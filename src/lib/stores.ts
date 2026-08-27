import {
    writable,
    // derived
} from "svelte/store";
import type { Book, FilterOptions, Config } from "./types";

export const books = writable<Book[]>([]);
export const filters = writable<FilterOptions>({});
export const currentBookId = writable<string | null>(null);
export const config = writable<Config | null>(null);

// TBA

/*
export const filteredBooks = derived([books, filters], ([$books, $filters]) => {
    // apply filtering logic (fuzzy search, etc.) – can also be done in backend
    // but we'll do client-side for responsiveness
    return filterBooks($books, $filters);
});
*/

// TBA
/*
function filterBooks(books: Book[], filters: FilterOptions): Book[] {

 }
*/
