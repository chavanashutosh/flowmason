import { PlayCircle, Shuffle, Sparkles, Clock, Calendar, TrendingUp, BookOpen, Settings } from '../components/ui/icons';

interface DocSection {
  id: string;
  title: string;
  icon: React.ReactNode;
  content: Array<{ heading: string; text: string }>;
}

export default function Documentation() {
  const sections: DocSection[] = [
    {
      id: 'getting-started',
      title: 'Getting Started',
      icon: <PlayCircle size={24} className="text-primary-600" />,
      content: [
        {
          heading: 'What is FlowMason?',
          text: 'FlowMason is a visual automation platform that allows you to build powerful workflows by connecting different services and APIs together.',
        },
        {
          heading: 'Your First Flow',
          text: '1. Navigate to Flows and click "Create Flow"\n2. Give your flow a name and description\n3. Add bricks to define your workflow steps\n4. Configure each brick with the required parameters\n5. Save and test your flow',
        },
      ],
    },
    {
      id: 'flows',
      title: 'Creating Flows',
      icon: <Shuffle size={24} className="text-primary-600" />,
      content: [
        {
          heading: 'Flow Builder',
          text: 'The visual flow builder lets you drag and connect nodes to create your automation workflow.',
        },
        {
          heading: 'Adding Bricks',
          text: 'Click "Add Brick" to see available integrations. Select a brick type and configure it.',
        },
        {
          heading: 'Testing Flows',
          text: 'Use the "Run Flow" button to test your flow immediately.',
        },
      ],
    },
    {
      id: 'templates',
      title: 'Using Templates',
      icon: <Sparkles size={24} className="text-primary-600" />,
      content: [
        {
          heading: 'Template Library',
          text: 'Browse pre-built templates to quickly get started with common automation patterns.',
        },
        {
          heading: 'Using a Template',
          text: '1. Go to Templates page\n2. Browse available templates\n3. Click "Use Template"\n4. Customize the flow',
        },
      ],
    },
    {
      id: 'executions',
      title: 'Monitoring Executions',
      icon: <Clock size={24} className="text-primary-600" />,
      content: [
        {
          heading: 'Execution History',
          text: 'View all flow executions in the Executions page.',
        },
        {
          heading: 'Execution Status',
          text: 'Executions can have the following statuses:\n• Pending\n• Running\n• Completed\n• Failed',
        },
      ],
    },
    {
      id: 'scheduler',
      title: 'Scheduling Flows',
      icon: <Calendar size={24} className="text-primary-600" />,
      content: [
        {
          heading: 'Cron Expressions',
          text: 'Schedule flows to run automatically using cron expressions.',
        },
        {
          heading: 'Creating a Schedule',
          text: '1. Go to Scheduler page\n2. Click "Schedule Flow"\n3. Select a flow\n4. Enter a cron expression',
        },
      ],
    },
    {
      id: 'metering',
      title: 'Usage & Metering',
      icon: <TrendingUp size={24} className="text-primary-600" />,
      content: [
        {
          heading: 'Usage Tracking',
          text: 'Monitor usage and quotas for each brick type.',
        },
        {
          heading: 'Quotas',
          text: 'Each brick type has daily and monthly usage limits.',
        },
      ],
    },
  ];

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold text-gray-900 flex items-center gap-3">
          <BookOpen size={32} className="text-primary-600" />
          Documentation
        </h1>
        <p className="text-gray-600 mt-2">Learn how to use FlowMason to build automation workflows</p>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {sections.map((section) => (
          <div key={section.id} className="bg-white rounded-lg shadow p-6 hover:shadow-lg transition-shadow">
            <div className="flex items-center gap-3 mb-4">
              <div className="p-2 bg-primary-50 rounded-lg">
                {section.icon}
              </div>
              <h2 className="text-xl font-bold text-gray-900">{section.title}</h2>
            </div>
            <div className="space-y-4">
              {section.content.map((item, idx) => (
                <div key={idx}>
                  <h3 className="font-semibold text-gray-900 mb-2">{item.heading}</h3>
                  <p className="text-gray-600 text-sm whitespace-pre-line">{item.text}</p>
                </div>
              ))}
            </div>
          </div>
        ))}
      </div>

      <div className="bg-primary-50 border border-primary-200 rounded-lg p-6">
        <h2 className="text-xl font-bold text-gray-900 mb-4 flex items-center gap-2">
          <Settings size={24} className="text-primary-600" />
          Quick Tips
        </h2>
        <ul className="space-y-2 text-gray-700">
          <li className="flex items-start gap-2">
            <span className="text-primary-600 font-bold">•</span>
            <span>Start with templates to learn common patterns</span>
          </li>
          <li className="flex items-start gap-2">
            <span className="text-primary-600 font-bold">•</span>
            <span>Test flows before scheduling them</span>
          </li>
          <li className="flex items-start gap-2">
            <span className="text-primary-600 font-bold">•</span>
            <span>Monitor execution history to debug issues</span>
          </li>
          <li className="flex items-start gap-2">
            <span className="text-primary-600 font-bold">•</span>
            <span>Check usage limits in Metering before production</span>
          </li>
        </ul>
      </div>
    </div>
  );
}
