import type { Metadata } from 'next';
import './globals.css';
import { AdminLayout } from '@/components/layouts/AdminLayout';

export const metadata: Metadata = {
  title: 'FlowMason - Visual Automation Platform',
  description: 'Build automation flows with bricks',
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" suppressHydrationWarning>
      <body>
        <AdminLayout>{children}</AdminLayout>
      </body>
    </html>
  );
}

