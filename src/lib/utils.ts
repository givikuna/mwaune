import * as Fuse from "fuse.js";

import { pipe } from "fp-ts/function";
import { fold } from "fp-ts/Option";

import { Book, BookType, FilterOptions, ReadStatus } from "./types";

export function filterBooks(books: Book[], filters: FilterOptions): Book[] {
    let result: Book[] = books;

    pipe(
        filters.query,
        fold(
            (): void => {},
            (query: string): void => {
                const fuse: Fuse.default<Book> = new Fuse.default(result, {
                    keys:      ["title", "authors"],
                    threshold: 0.3,
                });
                result = fuse.search(query).map((r: Fuse.FuseResult<Book>): Book => r.item);
            },
        ),
    );

    pipe(
        filters.language,
        fold(
            (): void => {},
            (lang: string): void => {
                result = result.filter((b: Book): boolean => b.language === lang);
            },
        ),
    );

    pipe(
        filters.book_type,
        fold(
            (): void => {},
            (type: BookType): void => {
                result = result.filter((b: Book): boolean => b.book_type === type);
            },
        ),
    );

    pipe(
        filters.read_status,
        fold(
            (): void => {},
            (status: ReadStatus) => {
                result = result.filter((b: Book): boolean => {
                    switch (status) {
                        case ReadStatus.Read:
                            return b.read === true;
                        case ReadStatus.Unread:
                            return b.read == false && b.progress.page === 0;
                        case ReadStatus.InProgress:
                            return b.read == false && b.progress.page > 0;
                        default:
                            return true;
                    }
                });
            },
        ),
    );

    pipe(
        filters.year_range,
        fold(
            (): void => {},
            ([min, max]: [number, number]): void => {
                result = result.filter((b: Book): boolean =>
                    pipe(
                        b.year,
                        fold(
                            (): boolean => false,
                            (y: number): boolean => y >= min && y <= max,
                        ),
                    ),
                );
            },
        ),
    );

    // 7. Page range
    pipe(
        filters.page_range,
        fold(
            (): void => {},
            ([min, max]: [number, number]) => {
                result = result.filter((b: Book): boolean =>
                    pipe(
                        b.pages,
                        fold(
                            (): boolean => false,
                            (p: number): boolean => p >= min && p <= max,
                        ),
                    ),
                );
            },
        ),
    );

    return result;
}
