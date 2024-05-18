import { writable } from 'svelte/store';
import {navigate} from 'svelte-navigator';
import {user} from '$stores';

export const deleteDialogOpen = writable(false);

export const isCreating = writable(false);
export const isEditing = writable(false);
export const isViewing = writable(false);

export const fieldErrors = writable({ first_name: '', last_name: '', email: '', role: ''});
export const hasErrors = writable(false);

export const id = writable(null);

export const form = writable({
    id: '',
    first_name: '',
    last_name: '',
    email: '',
    phone: '',
    role: '',
    active: '',
    last_login: '',
    occupation: '',
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
    const unsubscribe = user.subscribe(value => {
        form.update(formValue => {
            formValue.id = value?.id;
            formValue.first_name = value?.first_name;
            formValue.last_name = value?.last_name;
            formValue.email = value?.email;
            formValue.phone = value?.phone;
            formValue.role = value?.role?.id;
            formValue.active = value?.active;
            formValue.last_login = new Date(value?.last_login)?.toLocaleString();
            formValue.occupation = value?.occupation;
            formValue.facility = value?.facility?.id;
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
    first_name: z.string().min(1, 'First name is required').max(255, 'First name must be 255 characters or less'),
    last_name: z.string().min(1, 'Last name is required').max(255, 'Last name must be 255 characters or less'),
    email: z.string().min(1, 'Email is required').max(255, 'Email must be 255 characters or less').email('Must be a valid email'),
    role: z.string().min(1, 'Role must be set').uuid('Must be a valid role'),
});