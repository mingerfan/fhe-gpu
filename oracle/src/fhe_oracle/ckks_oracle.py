"""CKKS oracle: reads JSON from stdin, runs OpenFHE, writes JSON to stdout.

Protocol
--------
stdin:
    {
      "operation": "add_ct_ct",
      "inputs": {"x": [[1.5, 0.0], ...], "y": [[0.5, 0.0], ...]},
      "params": {"poly_degree": 4096, "scale_bits": 40, "depth": 3}
    }

stdout:
    {
      "result": [[2.0, 0.0], ...],
      "scale": 1099511627776.0
    }

Supported operations
--------------------
- add_ct_ct
- sub_ct_ct
- mul_ct_ct  (includes relin + rescale)
- add_ct_pt
- mul_ct_pt
- rotate (requires "step" in inputs)
- negate
"""

import json
import sys
from typing import Any

from .params import make_crypto_context
from .serialization import slots_to_list, list_to_slots


class CkksOracle:
    """Stateful CKKS oracle wrapping an OpenFHE CryptoContext."""

    def __init__(self, poly_degree: int, scale_bits: int, depth: int) -> None:
        self.cc, self.keys = make_crypto_context(poly_degree, scale_bits, depth)

    def _encode_encrypt(self, values: list[list[float]]) -> Any:
        """Encode and encrypt a list of [re, im] pairs."""
        import openfhe
        slots = list_to_slots(values)
        pt = self.cc.MakeCKKSPackedPlaintext(slots)
        return self.cc.Encrypt(self.keys.publicKey, pt)

    def _decrypt_decode(self, ct: Any) -> list[list[float]]:
        """Decrypt and decode a ciphertext back to [re, im] pairs."""
        result = self.cc.Decrypt(ct, self.keys.secretKey)
        result.SetLength(len(result.GetCKKSPackedValue()))
        slots = result.GetCKKSPackedValue()
        return slots_to_list(slots)

    def run(self, request: dict) -> dict:
        op = request["operation"]
        params = request.get("params", {})
        inputs = request.get("inputs", {})

        if op == "add_ct_ct":
            ct_x = self._encode_encrypt(inputs["x"])
            ct_y = self._encode_encrypt(inputs["y"])
            ct_r = self.cc.EvalAdd(ct_x, ct_y)
            result = self._decrypt_decode(ct_r)

        elif op == "sub_ct_ct":
            ct_x = self._encode_encrypt(inputs["x"])
            ct_y = self._encode_encrypt(inputs["y"])
            ct_r = self.cc.EvalSub(ct_x, ct_y)
            result = self._decrypt_decode(ct_r)

        elif op == "mul_ct_ct":
            ct_x = self._encode_encrypt(inputs["x"])
            ct_y = self._encode_encrypt(inputs["y"])
            ct_r = self.cc.EvalMult(ct_x, ct_y)  # includes relin + rescale
            result = self._decrypt_decode(ct_r)

        elif op == "negate":
            ct_x = self._encode_encrypt(inputs["x"])
            ct_r = self.cc.EvalNegate(ct_x)
            result = self._decrypt_decode(ct_r)

        elif op == "rotate":
            step = int(inputs.get("step", 1))
            ct_x = self._encode_encrypt(inputs["x"])
            ct_r = self.cc.EvalRotate(ct_x, step)
            result = self._decrypt_decode(ct_r)

        else:
            raise ValueError(f"Unknown operation: {op!r}")

        scale = 2.0 ** params.get("scale_bits", 40)
        return {"result": result, "scale": scale}


def run_oracle() -> None:
    """Main entry point: read JSON from stdin, write JSON to stdout."""
    request = json.load(sys.stdin)
    p = request.get("params", {})
    oracle = CkksOracle(
        poly_degree=p.get("poly_degree", 4096),
        scale_bits=p.get("scale_bits", 40),
        depth=p.get("depth", 3),
    )
    response = oracle.run(request)
    json.dump(response, sys.stdout)
    sys.stdout.flush()


if __name__ == "__main__":
    run_oracle()
