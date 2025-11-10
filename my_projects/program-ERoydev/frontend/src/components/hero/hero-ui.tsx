const Hero = () => {
  return (
    <div className="hero min-h-screen">
      <div className="hero-content flex flex-col-reverse items-center justify-center gap-8 lg:flex-row-reverse lg:items-center lg:gap-16 w-full px-4 md:px-8">
        <div className="hidden md:block w-48 h-48 sm:w-64 sm:h-64 md:w-80 md:h-80 lg:w-80 lg:h-80 xl:w-[28rem] xl:h-[28rem] cursor-pointer relative rounded-full overflow-hidden border-8 border-primary shadow-2xl group aspect-square flex-shrink-0 mb-6 lg:mb-0">
          <img
            src="hero.jpeg"
            alt="Hero"
            className="w-full h-full object-cover rounded-full transition-transform duration-500 group-hover:scale-110 group-hover:rotate-3"
          />
        </div>
        <div className="w-full max-w-xl text-center lg:text-left flex flex-col items-center lg:items-start">
          <h1 className="text-3xl sm:text-4xl md:text-5xl font-bold">Solana Liquidity Hub!</h1>
          <p className="py-4 sm:py-6 text-gray-400 text-base sm:text-lg md:text-xl">
            A fast, secure, and decentralized Automated Market Maker (AMM) built on the Solana blockchain using the
            Anchor framework. Swap tokens, provide liquidity, and experience lightning-fast transactions with low fees.
          </p>
          <button className="btn btn-primary w-full sm:w-auto">Get Started</button>
        </div>
      </div>
    </div>
  )
}

export default Hero
