import { writable } from 'svelte/store';
import {navigate} from 'svelte-navigator';
import {report} from '$stores';

export const deleteDialogOpen = writable(false);

export const isCreating = writable(false);
export const isEditing = writable(false);
export const isViewing = writable(false);

export const fieldErrors = writable({ title: '', description: '', report_type: '', status: ''});
export const hasErrors = writable(false);

export const id = writable(null);

export const form = writable({
    id: '',
    title: '',
    description: '',
    report_type: '',
    status: '',
    archived: false,
    creator: '',
    machine: '',
    created: '',
    edited: '',
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
    const unsubscribe = report.subscribe(value => {
        form.update(formValue => {
            formValue.id = value?.id;
            formValue.title = value?.title;
            formValue.description = value?.description;
            formValue.report_type = value?.report_type?.id;
            formValue.status = value?.status?.id;
            formValue.archived = value?.archived;
            formValue.creator = value?.creator;
            formValue.machine = value?.machine?.id;
            formValue.created = new Date(value?.created)?.toLocaleString();
            formValue.edited = new Date(value?.edited)?.toLocaleString();
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
    report_type: z.string().min(1, 'Type must be set').uuid('Must be a valid type'),
    status: z.string().min(1, 'Status must be set').uuid('Must be a valid status'),
});