export function getTokenValue(amount: number, price: number) {
  return amount * price;
}

export function getTotalDeposit(baseAmount: number, basePrice: number, quoteAmount: number, quotePrice: number) {
  return getTokenValue(baseAmount, basePrice) + getTokenValue(quoteAmount, quotePrice);
}

export function getTokenRatio(tokenValue: number, totalDeposit: number) {
  return totalDeposit ? Math.round((tokenValue / totalDeposit) * 100) : 0;
}