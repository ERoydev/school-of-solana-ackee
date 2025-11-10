import { AppProviders } from '@/components/app-providers.tsx'
import { AppLayout } from '@/components/app-layout.tsx'
import { RouteObject, useRoutes } from 'react-router'
import { lazy } from 'react'


const links = [
  //
  { label: 'Home', path: '/' },
  { label: 'Account', path: '/account' },
  { label: 'Liquidity', path: '/pools' },
  { label: 'Portfolio', path: '/portfolio' },
  { label: 'Mint Tokens', path: '/mint' },
]

// make other components lazy loaded
import PoolsList from './components/pools/pools-list-ui.tsx'
import CreatePool from './components/create-pool/create-pool-ui.tsx'
import MintTokens from './components/mint/mint-tokens-ui.tsx'
import { Portfolio } from './components/portfolio/portfolio.tsx'
const LazyHero = lazy(() => import('@/components/hero/hero-ui'))
const LazyAccountIndex = lazy(() => import('@/components/account/account-index-feature'))
const LazyAccountDetail = lazy(() => import('@/components/account/account-detail-feature'))
const LazyCounter = lazy(() => import('@/components/counter/counter-feature'))

const routes: RouteObject[] = [
  { index: true, element: <LazyHero /> },
  {
    path: 'account',
    children: [
      { index: true, element: <LazyAccountIndex /> },
      { path: ':address', element: <LazyAccountDetail /> },
    ],
  },
  { path: 'counter', element: <LazyCounter /> },
  { path: 'pools', element: <PoolsList />},
  { path: 'create-pool', element: <CreatePool/> },
  { path: 'mint', element: <MintTokens /> },
  { path: 'portfolio', element: <Portfolio />}
]

export function App() {
  const router = useRoutes(routes)
  return (
    <AppProviders>
      <AppLayout links={links}>{router}</AppLayout>
    </AppProviders>
  )
}
