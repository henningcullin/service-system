import { writable } from 'svelte/store';
import {navigate} from 'svelte-navigator';
import {facility} from '$stores';

export const deleteDialogOpen = writable(false);

export const isCreating = writable(false);
export const isEditing = writable(false);
export const isViewing = writable(false);

export const fieldErrors = writable({ name: '' });
export const hasErrors = writable(false);

export const id = writable(null);

export const form = writable({
    id: '',
    name: '',
    address: '',
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
    const unsubscribe = facility.subscribe(value => {
        form.update(formValue => {
            formValue.id = value?.id;
            formValue.name = value?.name;
            formValue.address = value?.address;
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
    name: z.string().min(1, 'Name is required').max(255, 'Name must be 255 characters or less'),
});
