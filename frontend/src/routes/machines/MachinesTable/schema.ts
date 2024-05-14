import { z } from "zod";

// We're keeping a simple non-relational schema here.
// IRL, you will have a schema for your data models.

export const machineSchema = z.object({
    id: z.string(),
    name: z.string(),
    make: z.string().nullable(),
    machine_type: z.object({
        id: z.string(),
        name: z.string(),
    }),
    status: z.object({
        id: z.string(),
        name: z.string(),
    }),
    created: z.string(),
    edited: z.string(),
    facility: z.object({
        id: z.string().nullable(),
        name: z.string().nullable(),
        address: z.string().nullable(),
    }),
    image: z.string().nullable(),
});

export type Machine = z.infer<typeof machineSchema>;