import { invoke } from "@tauri-apps/api/core";

import { Book, AddBookPayload, FilterOptions, AppError } from "./types";

import * as TE from "fp-ts/TaskEither";
import * as O from "fp-ts/Option";

const toAppError = (error: unknown): AppError => ({
    kind:    "Unknown",
    message: error instanceof Error ? error.message : String(error),
});

const serializePayload = (payload: AddBookPayload) => ({
    ...payload,
    title:         O.toNullable(payload.title),
    authors:       O.toNullable(payload.authors),
    genre:         O.toNullable(payload.genre),
    language:      O.toNullable(payload.language),
    year:          O.toNullable(payload.year),
    academic_meta: O.toNullable(payload.academic_meta),
});

export const getBooks = (filters: O.Option<FilterOptions>): TE.TaskEither<AppError, Book[]> =>
    TE.tryCatch(
        (): Promise<Book[]> => invoke<Book[]>("get_books", { filters: O.toNullable(filters) }),
        (e: unknown): AppError => toAppError(e),
    );

export const addBook = (payload: AddBookPayload): TE.TaskEither<AppError, Book> =>
    TE.tryCatch(
        (): Promise<Book> => invoke<Book>("add_book", { payload: serializePayload(payload) }),
        (e: unknown) => toAppError(e),
    );

export const deleteBook = (id: string): TE.TaskEither<AppError, void> =>
    TE.tryCatch(
        (): Promise<void> => invoke<void>("delete_book", { id }),
        (e: unknown): AppError => toAppError(e),
    );
