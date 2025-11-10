import { Button } from "../ui/button"
import { useNavigate } from "react-router-dom"
import PoolRow from "./pool-row-ui"

// Mock data for pools with liquidity, fees, and APR
const pools = [
  {
    id: 1,
    tokenA: 'SOL',
    tokenB: 'USDC',
    liquidity: 1200000,
    fees24h: 3200,
    apr24h: 8.2,
  },
  {
    id: 2,
    tokenA: 'BTC',
    tokenB: 'SOL',
    liquidity: 800000,
    fees24h: 2100,
    apr24h: 6.5,
  },
  {
    id: 3,
    tokenA: 'ETH',
    tokenB: 'USDT',
    liquidity: 950000,
    fees24h: 1800,
    apr24h: 7.1,
  },
]

export default function PoolsList() {
  const navigate = useNavigate();
  return (
    <div className="overflow-x-auto p-4">
      <div className="mb-8 flex flex-col gap-8">
        <div>
          <h2 className="text-xl md:text-2xl lg:text-3xl font-extrabold text-cyan-400 mb-2 leading-tight">Liquidity Pools</h2>
          <p className="text-gray-400 text-sm md:text-base lg:text-lg leading-relaxed">Provide liquidity, earn yield.</p>
        </div>

        <div className="bg-gradient-to-r from-cyan-700 via-blue-800 to-purple-700 rounded-2xl p-6 flex flex-col md:flex-row items-center justify-between shadow-lg mb-6">
          <div>
            <h2 className="text-xl md:text-2xl lg:text-2xl font-bold text-white mb-2 leading-tight">Welcome to the Liquidity Pools!</h2>
            <p className="text-cyan-100 text-xs md:sm lg:text-md max-w-xl leading-relaxed">Add your tokens to a pool and earn yield from trading fees. Creating a pool helps grow the ecosystem and gives you a share of the rewards.</p>
          </div>
          <Button
            className="mt-4 md:mt-0 px-6 py-3 rounded-xl bg-cyan-400 text-black font-bold text-base md:text-lg shadow hover:bg-cyan-300 transition"
            onClick={() => navigate("/create-pool")}
          >
            Create Liquidity Pool
          </Button>
        </div>
      </div>

    <table className="min-w-full bg-gray-900 rounded-2xl window-shadow mt-10 text-xs md:text-sm lg:text-base">
        <thead>
          <tr className="bg-gray-800 text-cyan-300">
            <th className="py-3 px-4 text-left font-semibold">Pool</th>
            <th className="py-3 px-4 text-left font-semibold">Liquidity</th>
            <th className="py-3 px-4 text-left font-semibold">Fees (24H)</th>
            <th className="py-3 px-4 text-left font-semibold">APR (24H)</th>
            <th className="py-3 px-4 text-center font-semibold">Actions</th>
          </tr>
        </thead>
        <tbody>
          {pools.map((pool) => (
            <PoolRow key={pool.id} pool={pool} />
          ))}
        </tbody>
      </table>
    </div>
  )
}
