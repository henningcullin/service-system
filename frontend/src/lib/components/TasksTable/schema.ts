import { z } from "zod";

// We're keeping a simple non-relational schema here.
// IRL, you will have a schema for your data models.

export const taskSchema = z.object({
    id: z.string(),
    title: z.string(),
    description: z.string(),
    task_type: z.object({
        id: z.string(),
        name: z.string(),
    }),
    status: z.object({
        id: z.string(),
        name: z.string(),
    }),
    archived: z.boolean(),
    creator: z.object({
        id: z.string(),
        first_name: z.string(),
        last_name: z.string(),
        email: z.string(),
        image: z.string().nullable(),
    }),
    executors: z.array(z.object({
        id: z.string(),
        first_name: z.string(),
        last_name: z.string(),
        email: z.string(),
        image: z.string().nullable(),
    })).nullable(),
    machine: z.object({
        id: z.string().nullable(),
        name: z.string().nullable(),
        make: z.string().nullable(),
        image: z.string().nullable(),
    }),
    created: z.string(),
    edited: z.string(),
    due_at: z.string().nullable(),
});

export type Task = z.infer<typeof taskSchema>;