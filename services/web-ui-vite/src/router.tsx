import { Routes, Route } from 'react-router-dom'
import AdminLayout from './components/layouts/AdminLayout'
import Dashboard from './pages/Dashboard'
import Flows from './pages/Flows'
import FlowDetail from './pages/FlowDetail'
import NewFlow from './pages/NewFlow'
import EditFlow from './pages/EditFlow'
import Templates from './pages/Templates'
import Executions from './pages/Executions'
import Scheduler from './pages/Scheduler'
import Metering from './pages/Metering'
import Mapping from './pages/Mapping'
import Documentation from './pages/Documentation'
import Settings from './pages/Settings'

export default function Router() {
  return (
    <Routes>
      <Route path="/" element={<AdminLayout />}>
        <Route index element={<Dashboard />} />
        <Route path="flows" element={<Flows />} />
        <Route path="flows/:id" element={<FlowDetail />} />
        <Route path="flows/new" element={<NewFlow />} />
        <Route path="flows/:id/edit" element={<EditFlow />} />
        <Route path="templates" element={<Templates />} />
        <Route path="executions" element={<Executions />} />
        <Route path="scheduler" element={<Scheduler />} />
        <Route path="metering" element={<Metering />} />
        <Route path="mapping" element={<Mapping />} />
        <Route path="documentation" element={<Documentation />} />
        <Route path="settings" element={<Settings />} />
      </Route>
    </Routes>
  )
}
