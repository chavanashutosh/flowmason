import { useState, useEffect } from 'react';
import { X } from './icons';

const ONBOARDING_DISMISSED_KEY = 'flowmason_onboarding_dismissed';

export default function OnboardingPanel() {
  const [isDismissed, setIsDismissed] = useState(false);

  useEffect(() => {
    const dismissed = localStorage.getItem(ONBOARDING_DISMISSED_KEY) === 'true';
    setIsDismissed(dismissed);
  }, []);

  const dismiss = () => {
    localStorage.setItem(ONBOARDING_DISMISSED_KEY, 'true');
    setIsDismissed(true);
  };

  if (isDismissed) {
    return null;
  }

  return (
    <div className="bg-gray-50 border border-gray-200 rounded-lg p-4 mb-6 flex items-start justify-between">
      <div className="flex-1">
        <p className="text-sm text-gray-700">Create a flow or browse templates to get started.</p>
      </div>
      <button
        onClick={dismiss}
        className="ml-4 text-gray-400 hover:text-gray-600 transition-colors flex-shrink-0"
      >
        <X size={20} />
      </button>
    </div>
  );
}
