import { writable } from 'svelte/store';
import {navigate} from 'svelte-navigator';
import {task} from '$stores';
import {CalendarDate, GregorianCalendar, parseDate} from '@internationalized/date';

export const deleteDialogOpen = writable(false);

export const isCreating = writable(false);
export const isEditing = writable(false);
export const isViewing = writable(false);

export const fieldErrors = writable({ title: '', description: '', task_type: '', status: '', archived: '', executors:'', machine:'', due_at:''});
export const hasErrors = writable(false);

export const id = writable(null);

export const form = writable({
    id: '',
    title: '',
    description: '',
    task_type: '',
    status: '',
    archived: false,
    creator: '',
    executors: [],
    machine: '',
    created: '',
    edited: '',
    due_at: '',
});

export function clearFields() {
    form.update(value => {
        for (const field in value) {
            value[field] = '';
        }
        return value;
    });
}

export function loadFields() {
    const unsubscribe = task.subscribe(value => {

        const due_at_date = value?.due_at ? new Date(value?.due_at) : null;
        const due_at = due_at_date ? new CalendarDate(new GregorianCalendar(), due_at_date.getUTCFullYear(), due_at_date.getUTCMonth() + 1, due_at_date.getUTCDate()) : null;

        form.update(formValue => {
            formValue.id = value?.id;
            formValue.title = value?.title;
            formValue.description = value?.description;
            formValue.task_type = value?.task_type?.id;
            formValue.status = value?.status?.id;
            formValue.archived = value?.archived;
            formValue.creator = value?.creator;
            formValue.executors = value?.executors instanceof Array ? value?.executors?.map(exec => exec?.id) : [];
            formValue.machine = value?.machine?.id;
            formValue.created = new Date(value?.created)?.toLocaleString();
            formValue.edited = new Date(value?.edited)?.toLocaleString();
            formValue.due_at =  due_at ?? null;
            return formValue;
        });
    });
    unsubscribe();
}


export function updateUrl(id = null) {
    const url = new URL(window.location.href);
    const pathArray = url.pathname.split('/');
    if (pathArray.length > 2) pathArray.pop();
    if (id) pathArray.push(id);
    url.pathname = pathArray.join('/');
    const newUrl = url.href;
    navigate(newUrl);
}

import { z } from 'zod';

export const formSchema = z.object({
    title: z.string().min(1, 'Title is required').max(255, 'Title must be 255 characters or less'),
    description: z.string().min(1, 'Description is required').max(255, 'Description must be 255 characters or less'),
    task_type: z.string().min(1, 'Type must be set').uuid('Must be a valid type'),
    status: z.string().min(1, 'Status must be set').uuid('Must be a valid status'),
    archived: z.boolean().nullable(),
    executors: z.array(z.string().uuid().nullable()).nullable(),
    machine: z.string().nullable()
});

export function due_at_converter(source) {
    return source ? new Date(`${source?.month}-${source?.day}-${source?.year}`).toISOString() : null;
}