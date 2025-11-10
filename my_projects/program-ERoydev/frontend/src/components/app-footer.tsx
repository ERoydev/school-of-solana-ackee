export function AppFooter() {
  return (
    <footer className="w-full bg-gradient-to-r from-cyan-900 via-blue-900 to-purple-900 py-6 px-4 flex flex-col md:flex-row items-center justify-between text-cyan-100 text-sm mt-12 shadow-inner">
      <div className="flex items-center gap-2">
        <svg width="24" height="24" fill="none" viewBox="0 0 24 24" className="inline-block mr-2 text-cyan-400"><circle cx="12" cy="12" r="12" fill="currentColor" /></svg>
        <span className="font-semibold">Solana dApp Demo</span>
      </div>
      <div className="flex items-center gap-2 mt-2 md:mt-0">
        <span>Created by</span>
        <a
          className="underline hover:text-cyan-300 transition"
          href="https://github.com/ERoydev"
          target="_blank"
          rel="noopener noreferrer"
        >
          E.Roydev
        </a>
      </div>
    </footer>
  )
}
