'use client';

import { Table, Spinner } from 'flowbite-react';
import { ReactNode } from 'react';

interface Column<T> {
  header: string;
  accessor: keyof T | ((row: T) => ReactNode);
  className?: string;
}

interface DataTableProps<T> {
  columns: Column<T>[];
  data: T[];
  loading?: boolean;
  emptyMessage?: string;
  onRowClick?: (row: T) => void;
  className?: string;
}

export function DataTable<T extends Record<string, any>>({
  columns,
  data,
  loading = false,
  emptyMessage = 'No data available',
  onRowClick,
  className = '',
}: DataTableProps<T>) {
  if (loading) {
    return (
      <div className="flex items-center justify-center h-64">
        <Spinner size="xl" />
      </div>
    );
  }

  if (data.length === 0) {
    return (
      <div className="text-center py-12 text-gray-500 dark:text-gray-400">
        {emptyMessage}
      </div>
    );
  }

  return (
    <div className={`overflow-x-auto ${className}`}>
      <Table hoverable={!!onRowClick}>
        <Table.Head>
          {columns.map((column, index) => (
            <Table.HeadCell key={index} className={column.className}>
              {column.header}
            </Table.HeadCell>
          ))}
        </Table.Head>
        <Table.Body className="divide-y">
          {data.map((row, rowIndex) => (
            <Table.Row
              key={rowIndex}
              className={`bg-white dark:border-gray-700 dark:bg-gray-800 ${
                onRowClick ? 'cursor-pointer' : ''
              }`}
              onClick={() => onRowClick?.(row)}
            >
              {columns.map((column, colIndex) => {
                const content =
                  typeof column.accessor === 'function'
                    ? column.accessor(row)
                    : row[column.accessor];
                return (
                  <Table.Cell key={colIndex} className={column.className}>
                    {content}
                  </Table.Cell>
                );
              })}
            </Table.Row>
          ))}
        </Table.Body>
      </Table>
    </div>
  );
}

