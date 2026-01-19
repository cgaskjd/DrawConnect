import { useNavigationStore } from './stores/navigationStore'
import LandingPage from './components/LandingPage'
import PluginCommunityPage from './components/plugins/PluginCommunityPage'
import './styles/global.css'
import './styles/landing.css'

function App() {
  const { pageMode } = useNavigationStore()

  if (pageMode === 'plugins') {
    return <PluginCommunityPage />
  }

  return <LandingPage />
}

export default App
