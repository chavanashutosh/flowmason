'use client';

import { useEffect, useState } from 'react';
import { Card, Table, Button, Badge, Modal, Spinner } from 'flowbite-react';
import { Plus, Trash2, Edit, Eye } from 'lucide-react';
import Link from 'next/link';
import { api } from '@/lib/api';
import { StatusBadge } from '@/components/ui/StatusBadge';

export default function FlowsPage() {
  const [flows, setFlows] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);
  const [deleteModalOpen, setDeleteModalOpen] = useState(false);
  const [selectedFlow, setSelectedFlow] = useState<string | null>(null);

  useEffect(() => {
    fetchFlows();
  }, []);

  const fetchFlows = async () => {
    try {
      const data = await api.flows.list();
      setFlows(data);
    } catch (error) {
      console.error('Failed to fetch flows:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleDelete = async () => {
    if (!selectedFlow) return;
    try {
      await api.flows.delete(selectedFlow);
      setFlows(flows.filter(f => f.id !== selectedFlow));
      setDeleteModalOpen(false);
      setSelectedFlow(null);
    } catch (error) {
      console.error('Failed to delete flow:', error);
    }
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center h-64">
        <Spinner size="xl" />
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white">Flows</h1>
          <p className="text-gray-600 dark:text-gray-400 mt-1">
            Manage your automation flows
          </p>
        </div>
        <Link href="/flows/new">
          <Button gradientDuoTone="purpleToBlue" icon={Plus}>
            Create Flow
          </Button>
        </Link>
      </div>

      <Card>
        {flows.length === 0 ? (
          <div className="text-center py-12">
            <p className="text-gray-500 dark:text-gray-400 mb-4">No flows created yet</p>
            <Link href="/flows/new">
              <Button gradientDuoTone="purpleToBlue" icon={Plus}>
                Create Your First Flow
              </Button>
            </Link>
          </div>
        ) : (
          <Table hoverable>
            <Table.Head>
              <Table.HeadCell>Name</Table.HeadCell>
              <Table.HeadCell>Description</Table.HeadCell>
              <Table.HeadCell>Bricks</Table.HeadCell>
              <Table.HeadCell>Status</Table.HeadCell>
              <Table.HeadCell>Created</Table.HeadCell>
              <Table.HeadCell>
                <span className="sr-only">Actions</span>
              </Table.HeadCell>
            </Table.Head>
            <Table.Body className="divide-y">
              {flows.map((flow) => (
                <Table.Row key={flow.id} className="bg-white dark:border-gray-700 dark:bg-gray-800">
                  <Table.Cell className="font-medium text-gray-900 dark:text-white">
                    {flow.name}
                  </Table.Cell>
                  <Table.Cell className="text-gray-600 dark:text-gray-400">
                    {flow.description || '-'}
                  </Table.Cell>
                  <Table.Cell>
                    <Badge color="info">{flow.bricks?.length || 0} bricks</Badge>
                  </Table.Cell>
                  <Table.Cell>
                    <StatusBadge status={flow.active ? 'active' : 'inactive'} />
                  </Table.Cell>
                  <Table.Cell className="text-sm text-gray-600 dark:text-gray-400">
                    {new Date(flow.created_at).toLocaleDateString()}
                  </Table.Cell>
                  <Table.Cell>
                    <div className="flex items-center gap-2">
                      <Link href={`/flows/${flow.id}`}>
                        <Button size="xs" color="light" icon={Eye}>
                          View
                        </Button>
                      </Link>
                      <Link href={`/flows/${flow.id}`}>
                        <Button size="xs" color="light" icon={Edit}>
                          Edit
                        </Button>
                      </Link>
                      <Button
                        size="xs"
                        color="failure"
                        icon={Trash2}
                        onClick={() => {
                          setSelectedFlow(flow.id);
                          setDeleteModalOpen(true);
                        }}
                      >
                        Delete
                      </Button>
                    </div>
                  </Table.Cell>
                </Table.Row>
              ))}
            </Table.Body>
          </Table>
        )}
      </Card>

      <Modal show={deleteModalOpen} onClose={() => setDeleteModalOpen(false)}>
        <Modal.Header>Delete Flow</Modal.Header>
        <Modal.Body>
          <p className="text-gray-600 dark:text-gray-400">
            Are you sure you want to delete this flow? This action cannot be undone.
          </p>
        </Modal.Body>
        <Modal.Footer>
          <Button color="failure" onClick={handleDelete}>
            Delete
          </Button>
          <Button color="gray" onClick={() => setDeleteModalOpen(false)}>
            Cancel
          </Button>
        </Modal.Footer>
      </Modal>
    </div>
  );
}
