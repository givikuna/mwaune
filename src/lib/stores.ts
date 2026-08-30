import {
    writable,
    derived,
    //type Writable,
    type Readable,
} from "svelte/store";
// import { pipe } from "fp-ts/function";
import {
    Option,
    none,
    //some
} from "fp-ts/Option";
// import { Either, fold } from "fp-ts/Either";
import type { Book, FilterOptions, Config, AppError } from "./types";

export const books = writable<Book[]>([]);
export const isLoading = writable<boolean>(false);
export const error = writable<Option<AppError>>(none);

export const filters = writable<FilterOptions>({
    query:       none,
    genre:       none,
    language:    none,
    book_type:   none,
    read_status: none,
    year_range:  none,
    page_range:  none,
});

export const config = writable<Option<Config>>(none);

export const filteredBooks: Readable<Book[]> = derived([books, filters], ([$books, $filters]) => {
    // to-do fuse.js
    return applyFilters($books, $filters);
});

// helper
function applyFilters(books: Book[], _filters: FilterOptions): Book[] {
    // to-do fuse.js
    return books;
}
