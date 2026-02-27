export * from './auth';
export * from './tool';
export * from './skill';
export * from './snippet';

export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  meta?: {
    page?: number;
    perPage?: number;
    total?: number;
  };
  error?: {
    code: string;
    message: string;
  };
}

export interface PaginatedResponse<T> {
  items: T[];
  page: number;
  perPage: number;
  total: number;
}
