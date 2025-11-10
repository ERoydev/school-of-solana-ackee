import React, { useState } from "react";
import { getTokenValue, getTotalDeposit, getTokenRatio } from "@/utils/prices_utils";

const steps = [
  { title: "Select token & fee tier" },
  // { title: "Set initial price & range" }, # I could add that later but for education purposes i skip that for now
  { title: "Enter deposit amount" },
];

export default function CreatePool() {
  const [currentStep, setCurrentStep] = useState(0);
  const [baseToken, setBaseToken] = useState("RAY");
  const [quoteToken, setQuoteToken] = useState("USDC");
  const [feeTier, setFeeTier] = useState("0.18%");

  // Deposit state
  const [baseDeposit, setBaseDeposit] = useState(0);
  const [quoteDeposit, setQuoteDeposit] = useState(0);

  const submitHandler = () => {
    // Call the contract
    console.log('Submiting to create pool with:', {baseToken, quoteToken, feeTier, baseDeposit, quoteDeposit})
  }

  // Sync handlers
  // This handle the cluclation that when i deposit 1 base token i should get the quote token value which is 1,53 base on the current price
  const handleBaseDepositChange = (value: number) => {
    setBaseDeposit(value);
    setQuoteDeposit(Number(((value * baseTokenPrice) / quoteTokenPrice).toFixed(6)));
  };

  const handleQuoteDepositChange = (value: number) => {
    setQuoteDeposit(value);
    setBaseDeposit(Number(((value * quoteTokenPrice) / baseTokenPrice).toFixed(6)));
  };

  // Calculate total deposit and ratios
  // For example these are my token prices now hardcoded
  const baseTokenPrice = 2.3; // RAY price
  const quoteTokenPrice = 1.5; // USDC price

  // These are the calculation to display Deposit money and ratio and so on ===================
  const baseValue = getTokenValue(baseDeposit, baseTokenPrice);
  const quoteValue = getTokenValue(quoteDeposit, quoteTokenPrice);
  const totalDeposit = getTotalDeposit(baseDeposit, baseTokenPrice, quoteDeposit, quoteTokenPrice);

  const baseRatio = getTokenRatio(baseValue, totalDeposit);
  const quoteRatio = getTokenRatio(quoteValue, totalDeposit);
  // ======================================

  const nextStep = () => {
    if (currentStep < steps.length - 1) setCurrentStep(currentStep + 1);
  };

  const prevStep = () => {
    if (currentStep > 0) setCurrentStep(currentStep - 1);
  };

  return (
    <div className="flex min-h-screen justify-start text-white mt-12 w-full">
      <div className="w-full max-w-5xl p-6">
        <div className="flex flex-col md:flex-row gap-20">
          {/* Steps Indicator as a card */}
          <div className="md:w-1/3 w-full bg-gray-900 rounded-2xl p-6 flex flex-col gap-4 window-border window-shadow">
            <h3 className="mb-4 text-lg font-bold text-cyan-400">Steps</h3>
            {steps.map((step, index) => (
              <div key={index} className="flex items-center gap-3">
                <div
                  className={`flex h-9 w-9 items-center justify-center rounded-full border-2 text-base font-bold transition-colors ${
                    index === currentStep
                      ? "border-cyan-400 bg-cyan-400 text-black shadow"
                      : "border-gray-700 bg-gray-800 text-gray-400"
                  }`}
                >
                  {index + 1}
                </div>
                <span className={`font-medium ${index === currentStep ? "text-white" : "text-gray-400"}`}>
                  {step.title}
                </span>
              </div>
            ))}
          </div>

          {/* Step Content */}
          <div className="md:w-2/3 w-full rounded-2xl bg-gray-800 p-8 flex flex-col justify-between min-h-[400px] window-border window-shadow">
            {currentStep === 0 && (
              <div>
                <h2 className="mb-6 text-2xl font-semibold text-cyan-300">Select tokens & fee tier</h2>
                <div className="space-y-6">
                  <div className="grid grid-cols-2 gap-6">
                    <div>
                      <label className="mb-2 block text-sm text-gray-400">Base token</label>
                      <select
                        className="w-full rounded-lg bg-gray-900 p-3 focus:outline-none"
                        value={baseToken}
                        onChange={(e) => setBaseToken(e.target.value)}
                      >
                        <option value="RAY">RAY</option>
                        <option value="SOL">SOL</option>
                        <option value="BTC">BTC</option>
                      </select>
                    </div>
                    <div>
                      <label className="mb-2 block text-sm text-gray-400">Quote token</label>
                      <select
                        className="w-full rounded-lg bg-gray-900 p-3 focus:outline-none"
                        value={quoteToken}
                        onChange={(e) => setQuoteToken(e.target.value)}
                      >
                        <option value="USDC">USDC</option>
                        <option value="USDT">USDT</option>
                      </select>
                    </div>
                  </div>
                  <div>
                    <label className="mb-2 block text-sm text-gray-400">Fee Tier</label>
                    <select
                      className="w-full rounded-lg bg-gray-900 p-3 focus:outline-none"
                      value={feeTier}
                      onChange={(e) => setFeeTier(e.target.value)}
                    >
                      <option value="0.18%">0.18%</option>
                      <option value="0.30%">0.30%</option>
                      <option value="1%">1%</option>
                    </select>
                  </div>
                </div>
              </div>
            )}

            {currentStep === 1 && (
              <div className="flex flex-col gap-6">
                <h2 className="mb-2 text-2xl font-semibold text-cyan-300">Set initial price & range</h2>
                {/* Base Token Row */}
                <div className="flex items-center justify-between bg-gray-900 rounded-lg px-4 py-3 window-border window-shadow">
                  <span className="text-base font-medium text-gray-300">Base Token</span>
                  <div className="flex items-center gap-2">
                    <span className="text-lg font-bold text-cyan-400">{baseToken}</span>
                    <input
                      type="number"
                      min={0}
                      value={baseDeposit}
                      onChange={e => handleBaseDepositChange(Number(e.target.value))}
                      className="ml-4 w-40 rounded-lg bg-gradient-to-r from-gray-900 via-gray-800 to-cyan-900 px-4 py-2 text-right text-cyan-200 border-2 border-cyan-400/30 shadow focus:border-cyan-400 focus:ring-2 focus:ring-cyan-400/30 transition-all duration-200 placeholder:text-gray-500 outline-none"
                    />
                  </div>
                </div>
                {/* Quote Token Row */}
                <div className="flex items-center justify-between bg-gray-900 rounded-lg px-4 py-3 window-border window-shadow">
                  <span className="text-base font-medium text-gray-300">Quote Token</span>
                  <div className="flex items-center gap-2">
                    <span className="text-lg font-bold text-cyan-400">{quoteToken}</span>
                    <input
                      type="number"
                      min={0}
                      value={quoteDeposit}
                      onChange={e => handleQuoteDepositChange(Number(e.target.value))}
                      className="ml-4 w-40 rounded-lg bg-gradient-to-r from-gray-900 via-gray-800 to-cyan-900 px-4 py-2 text-right text-cyan-200 border-2 border-cyan-400/30 shadow focus:border-cyan-400 focus:ring-2 focus:ring-cyan-400/30 transition-all duration-200 placeholder:text-gray-500 outline-none"
                    />
                  </div>
                </div>
                {/* Deposit Info Window */}
                <div className="bg-gray-900 rounded-xl px-6 py-4 flex flex-col items-center window-border window-shadow">
                  <span className="text-base font-semibold text-gray-200 mb-2">Total Deposit</span>
                  <span className="text-2xl font-bold text-green-400 mb-2">${totalDeposit}</span>
                  <span className="text-sm text-gray-400">Deposit Ratio:</span>
                  <span className="text-sm text-cyan-300 mt-1">Base token {baseRatio}% / Quote token {quoteRatio}%</span>
                </div>
              </div>
            )}

            {/* Navigation */}
            <div className="mt-10 flex justify-between">
              <button
                onClick={prevStep}
                disabled={currentStep === 0}
                className="rounded-lg bg-gray-700 px-5 py-2 text-base text-gray-300 hover:bg-gray-600 disabled:opacity-40"
              >
                Back
              </button>
              {currentStep < steps.length - 1 ? (
                <button
                  onClick={nextStep}
                  className="rounded-lg bg-cyan-400 px-8 py-2 text-base font-semibold text-black hover:bg-cyan-300"
                >
                  Continue
                </button>
              ) : (
                <button onClick={submitHandler} className="rounded-lg bg-green-500 px-8 py-2 text-base font-semibold text-black hover:bg-green-400">
                  Submit
                </button>
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
