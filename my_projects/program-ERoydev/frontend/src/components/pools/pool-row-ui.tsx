import { Button } from "../ui/button";

interface PoolRowProps {
  pool: {
    id: number;
    tokenA: string;
    tokenB: string;
    liquidity: number;
    fees24h: number;
    apr24h: number;
  };
}

export default function PoolRow({ pool }: PoolRowProps) {
  return (
    <tr className="border-b border-cyan-400/10 hover:bg-gray-800/60 transition-all">
      <td className="py-3 px-4 font-bold text-cyan-200">{pool.tokenA}-{pool.tokenB}</td>
      <td className="py-3 px-4 text-green-300">${pool.liquidity.toLocaleString()}</td>
      <td className="py-3 px-4 text-cyan-300">${pool.fees24h.toLocaleString()}</td>
      <td className="py-3 px-4 text-purple-300">{pool.apr24h}%</td>
      <td className="py-3 px-4 flex gap-2 justify-center">
        <Button className="px-4 py-2 rounded-lg bg-cyan-500 text-black font-semibold shadow hover:bg-cyan-400 transition">
            Swap
        </Button>
        <Button className="px-4 py-2 rounded-lg bg-green-500 text-black font-semibold shadow hover:bg-green-400 transition">
            Deposit
        </Button>

      </td>
    </tr>
  );
}
