"""OpenFHE CryptoContext factory for CKKS."""

from __future__ import annotations
from typing import Tuple, Any


def make_crypto_context(
    poly_degree: int,
    scale_bits: int,
    depth: int,
) -> Tuple[Any, Any]:
    """Create and configure an OpenFHE CKKS CryptoContext.

    Returns
    -------
    (cc, keys) where cc is the CryptoContext and keys contains pk/sk/rlk.
    """
    import openfhe

    params = openfhe.CCParamsCKKSRNS()
    params.SetMultiplicativeDepth(depth)
    params.SetScalingModSize(scale_bits)
    params.SetBatchSize(poly_degree // 2)
    params.SetRingDim(poly_degree)

    cc = openfhe.GenCryptoContext(params)
    cc.Enable(openfhe.PKESchemeFeature.PKE)
    cc.Enable(openfhe.PKESchemeFeature.KEYSWITCH)
    cc.Enable(openfhe.PKESchemeFeature.LEVELEDSHE)

    keys = cc.KeyGen()
    cc.EvalMultKeyGen(keys.secretKey)
    cc.EvalRotateKeyGen(keys.secretKey, list(range(-poly_degree // 4, poly_degree // 4 + 1)))

    return cc, keys
