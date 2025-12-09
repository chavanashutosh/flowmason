'use client';

import { Modal, Button } from 'flowbite-react';
import { ReactNode } from 'react';

interface ConfirmModalProps {
  show: boolean;
  onClose: () => void;
  onConfirm: () => void;
  title: string;
  message: string | ReactNode;
  confirmText?: string;
  cancelText?: string;
  confirmColor?: 'failure' | 'success' | 'warning' | 'info' | 'gray';
  loading?: boolean;
}

export function ConfirmModal({
  show,
  onClose,
  onConfirm,
  title,
  message,
  confirmText = 'Confirm',
  cancelText = 'Cancel',
  confirmColor = 'failure',
  loading = false,
}: ConfirmModalProps) {
  return (
    <Modal show={show} onClose={onClose}>
      <Modal.Header>{title}</Modal.Header>
      <Modal.Body>
        <div className="text-gray-600 dark:text-gray-400">
          {typeof message === 'string' ? <p>{message}</p> : message}
        </div>
      </Modal.Body>
      <Modal.Footer>
        <Button color="gray" onClick={onClose} disabled={loading}>
          {cancelText}
        </Button>
        <Button color={confirmColor} onClick={onConfirm} disabled={loading}>
          {loading ? 'Processing...' : confirmText}
        </Button>
      </Modal.Footer>
    </Modal>
  );
}

