import { Link } from 'react-router-dom';
import { Clock, Play } from './icons';

export enum EmptyStateIcon {
  Clock = 'clock',
  Play = 'play',
}

interface EmptyStateProps {
  title: string;
  description: string;
  actionLabel?: string;
  actionRoute?: string;
  actionOnClick?: () => void;
  icon: EmptyStateIcon;
}

export default function EmptyState({
  title,
  description,
  actionLabel,
  actionRoute,
  actionOnClick,
  icon,
}: EmptyStateProps) {
  const renderIcon = () => {
    switch (icon) {
      case EmptyStateIcon.Clock:
        return <Clock size={48} className="text-gray-300" />;
      case EmptyStateIcon.Play:
        return <Play size={48} className="text-gray-300" />;
      default:
        return <Clock size={48} className="text-gray-300" />;
    }
  };

  return (
    <div className="flex flex-col items-center justify-center py-12 px-4 text-center">
      <div className="mb-4 text-gray-400">{renderIcon()}</div>
      <h3 className="text-lg font-semibold text-gray-900 mb-2">{title}</h3>
      <p className="text-sm text-gray-500 mb-6 max-w-sm">{description}</p>
      {actionLabel && (
        <>
          {actionRoute ? (
            <Link
              to={actionRoute}
              className="px-4 py-2 text-sm font-medium text-white bg-primary-600 hover:bg-primary-700 rounded-lg transition-colors inline-block"
            >
              {actionLabel}
            </Link>
          ) : actionOnClick ? (
            <button
              onClick={actionOnClick}
              className="px-4 py-2 text-sm font-medium text-white bg-primary-600 hover:bg-primary-700 rounded-lg transition-colors"
            >
              {actionLabel}
            </button>
          ) : null}
        </>
      )}
    </div>
  );
}
