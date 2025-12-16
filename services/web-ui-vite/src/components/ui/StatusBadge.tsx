export enum Status {
  Completed = 'completed',
  Failed = 'failed',
  Running = 'running',
  Pending = 'pending',
  Active = 'active',
  Inactive = 'inactive',
}

interface StatusBadgeProps {
  status: Status | string;
}

export default function StatusBadge({ status }: StatusBadgeProps) {
  const statusValue = typeof status === 'string' ? status : status;
  
  const getStatusConfig = () => {
    switch (statusValue) {
      case Status.Completed:
        return { colorClass: 'bg-green-100 text-green-800', label: 'Completed' };
      case Status.Active:
        return { colorClass: 'bg-green-100 text-green-800', label: 'Active' };
      case Status.Failed:
        return { colorClass: 'bg-red-100 text-red-800', label: 'Failed' };
      case Status.Running:
        return { colorClass: 'bg-blue-100 text-blue-800', label: 'Running' };
      case Status.Pending:
        return { colorClass: 'bg-yellow-100 text-yellow-800', label: 'Pending' };
      case Status.Inactive:
        return { colorClass: 'bg-gray-100 text-gray-800', label: 'Inactive' };
      default:
        return { colorClass: 'bg-gray-100 text-gray-800', label: statusValue };
    }
  };

  const { colorClass, label } = getStatusConfig();

  return (
    <span className={`px-2 py-1 text-xs font-semibold rounded ${colorClass}`}>
      {label}
    </span>
  );
}
