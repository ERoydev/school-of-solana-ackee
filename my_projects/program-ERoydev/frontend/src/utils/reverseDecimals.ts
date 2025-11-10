import BN from "bn.js";

/**
 * Convert token amount from smallest unit (BN) to human-readable number
 * @param amountBN - token amount as BN
 * @param decimals - number of decimals in the token mint (default 9)
 * @returns number
 */
export function fromDecimals(amountBN: BN, decimals = 9): number {
  return parseFloat(amountBN.toString()) / Math.pow(10, decimals);
}

/**
 * Optional: Return string with fixed decimal places
 */
export function fromDecimalsString(amountBN: BN, decimals = 9, fixed = 2): string {
  const amount = fromDecimals(amountBN, decimals);
  return amount.toFixed(fixed);
}
