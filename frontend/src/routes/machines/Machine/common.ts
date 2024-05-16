import { writable } from 'svelte/store';
import {navigate} from 'svelte-navigator';
import {machine} from '$stores';

export const deleteDialogOpen = writable(false);

export const isCreating = writable(false);
export const isEditing = writable(false);
export const isViewing = writable(false);

export const fieldErrors = writable({ name: '', make: '', machine_type: '', status: '', facility: '' });
export const hasErrors = writable(false);

export const id = writable(null);

export const form = writable({
    id: '',
    name: '',
    make: '',
    machine_type: '',
    status: '',
    created: '',
    edited: '',
    facility: '',
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
    const unsubscribe = machine.subscribe(value => {
        form.update(formValue => {
            formValue.id = value?.id;
            formValue.name = value?.name;
            formValue.make = value?.make;
            formValue.machine_type = value?.machine_type?.id;
            formValue.status = value?.status?.id;
            formValue.created = new Date(value?.created)?.toLocaleString();
            formValue.edited = new Date(value?.edited)?.toLocaleString();
            formValue.facility = value?.facility?.id;
            return formValue;
        });
    });
    unsubscribe();
}


export function updateUrl(location, id = null) {
    const url = new URL(location.href);
    const pathArray = url.pathname.split('/');
    if (pathArray.length > 2) pathArray.pop();
    if (id) pathArray.push(id);
    url.pathname = pathArray.join('/');
    const newUrl = url.href;
    navigate(newUrl);
}

import { z } from 'zod';

export const formSchema = z.object({
    name: z.string().min(1, 'Name is required').max(255, 'Name must be 255 characters or less'),
    make: z.string().max(255, 'Make must be 255 characters or less').nullable(),
    machine_type: z.string().min(1, 'Machine Type is required').uuid('Must be a valid Machine Type'),
    status: z.string().min(1, 'Status is required').uuid('Must be a valid Status'),
    facility: z.string().uuid('Must be a valid Facility').nullable(),
});
