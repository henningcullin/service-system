import { writable } from 'svelte/store';
import {navigate} from 'svelte-navigator';
import {facility} from '$stores';

export const deleteDialogOpen = writable(false);

export const isCreating = writable(false);
export const isEditing = writable(false);
export const isViewing = writable(false);

export const fieldErrors = writable({ name: '', level: '' });
export const hasErrors = writable(false);

export const id = writable(null);

export const form = writable({
    id: '',
    name: '',
    level: 0,
    has_password: true,
    user_view: false,
    user_create: false,
    user_edit: false,
    user_delete: false,
    machine_view: false,
    machine_create: false,
    machine_edit: false,
    machine_delete: false,
    task_view: false,
    task_create: false,
    task_edit: false,
    task_delete: false,
    report_view: false,
    report_create: false,
    report_edit: false,
    report_delete: false,
    facility_view: false,
    facility_create: false,
    facility_edit: false,
    facility_delete: false,
});

export function clearFields() {
    form.update(formState => {
        formState.id = '';
        formState.name = '';
        formState.level = 0;
        formState.has_password = true;
        for (const field in formState) {
            if (field !== 'id' && field !== 'name' && field !== 'level' && field !== 'has_password')
            formState[field] = false;
        }
        return formState;
    });
}

export function loadFields() {
    const unsubscribe = facility.subscribe(value => {
        form.set({...value});
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
  name: z.string().min(1, 'Name is required').max(255, 'Name must be at most 255 characters long'),
  level: z.number().min(0, 'Must be 0 or greater'),
});
