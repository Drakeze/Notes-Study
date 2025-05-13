// Common TypeScript interfaces and types
export interface User {
    id: string;
    name: string;
    email: string;
}

export type ApiResponse<T> = {
    data: T;
    status: number;
    message: string;
};

export type Theme = 'light' | 'dark';