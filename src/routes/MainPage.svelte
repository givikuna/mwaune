<script lang="ts">
    import { onMount } from "svelte";

    // import { open } from "@tauri-apps/plugin-dialog";

    import {
        //addBook,
        getBooks,
    } from "../lib/api";
    import { books, error, isLoading } from "../lib/stores";

    import {
        AppError,
        Book,
        // BookType
    } from "../lib/types";

    import * as TE from "fp-ts/TaskEither";
    import * as O from "fp-ts/Option";

    import { pipe } from "fp-ts/function";
    import { Task } from "fp-ts/lib/Task";

    // import { noop } from "underscore";

    /*
    const selectFile: TE.TaskEither<AppError, O.Option<string>> = pipe(
        TE.tryCatch(
            (): Promise<string | null> =>
                open({
                    multiple: false,
                    filters:  [{ name: "Books", extensions: ["pdf", "epub"] }],
                }),
            (reason: unknown): AppError => ({ kind: "FileSystem", message: String(reason) }),
        ),
        TE.map((result: string | null): O.Option<string> =>
            typeof result === "string" ? O.some(result) : O.none,
        ),
    );
    */

    const loadLibrary: Task<void> = pipe(
        getBooks(O.none),
        TE.match(
            (err: AppError): void => {
                $error = O.some(err);
                $isLoading = false;
            },
            (fetchedBooks: Book[]): void => {
                $books = fetchedBooks;
                $isLoading = false;
            },
        ),
    );

    onMount((): void => {
        $isLoading = true;
        loadLibrary();
    });

    /*
    const handleUpload: () => void = (): void => (
        pipe(
            selectFile,
            TE.flatMap(
                O.match(
                    (): TE.TaskEither<never, O.Option<never>> => TE.right(O.none),
                    (path: string): TE.TaskEither<AppError, O.Option<Book>> =>
                        pipe(
                            addBook({
                                file_path:     path,
                                title:         O.none,
                                authors:       O.none,
                                genre:         O.none,
                                language:      O.none,
                                year:          O.none,
                                book_type:     BookType.Book,
                                academic_meta: O.none,
                            }),
                            TE.map(O.some),
                        ),
                ),
            ),
            TE.match(
                (err: AppError): void => {
                    $error = O.some(err);
                },
                O.match(
                    (): void => noop(),
                    (newBook: Book): void => {
                        $books = [newBook, ...$books];
                    },
                ),
            ),
        )(),
        noop()
    );
    */

    const dismissError: () => void = (): void => {
        $error = O.none;
    };
</script>

<div class="tui-container gruvbox">
    {#if $error._tag === "Some"}
        <div class="error-banner">
            <span>[ERROR]: {$error.value.message}</span>
            <button
                class="tui-btn"
                on:click={dismissError}>[x]</button
            >
        </div>
    {/if}

    <main class="tui-main">
        {#if $isLoading}
            <p class="status">Loading library...</p>
        {:else if $books.length === 0}
            <p class="status">Library empty. Upload a file to begin.</p>
        {:else}
            <ul class="book-list">
                {#each $books as book (book.id)}
                    <li class="book-item">
                        <div class="book-info">
                            <span class="title">{book.title}</span>
                            <span class="author">
                                {book.authors.length > 0 ? `// ${book.authors[0]}` : "// Unknown"}
                            </span>
                        </div>
                        <div class="book-meta">
                            <span class="badge file-type">[{book.file_type}]</span>
                            <span class="badge read-status">
                                {book.read ? "[x]" : book.progress.page > 0 ? "[-]" : "[ ]"}
                            </span>
                        </div>
                    </li>
                {/each}
            </ul>
        {/if}
    </main>
</div>

<style>
    .tui-container {
        --bg: #282828;
        --fg: #ebdbb2;
        --red: #cc241d;
        --green: #98971a;
        --yellow: #d79921;
        --blue: #458588;
        --purple: #b16286;
        --aqua: #689d6a;
        --gray: #a89984;
        --bg-dark: #1d2021;

        background-color: var(--bg);
        color: var(--fg);
        min-height: 100vh;
        font-family: "JetBrains Mono", "Fira Code", monospace;
        padding: 2rem;
        box-sizing: border-box;
    }

    /*
    .tui-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        border-bottom: 2px solid var(--gray);
        padding-bottom: 1rem;
        margin-bottom: 2rem;
    }
        */

    /*
    h1 {
        margin: 0;
        font-size: 1.5rem;
        color: var(--green);
    }
        */

    .tui-btn {
        background: none;
        border: none;
        color: var(--gray);
        font-family: inherit;
        font-size: 1rem;
        cursor: pointer;
        padding: 0;
    }

    .tui-btn:hover {
        color: var(--fg);
    }

    /*
    .tui-btn.primary {
        color: var(--yellow);
    }

    .tui-btn.primary:hover {
        color: var(--green);
    }
        */

    .error-banner {
        background-color: var(--red);
        color: var(--bg);
        padding: 0.5rem 1rem;
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 1rem;
        font-weight: bold;
    }

    .error-banner .tui-btn {
        color: var(--bg);
    }

    .status {
        color: var(--gray);
        font-style: italic;
    }

    .book-list {
        list-style: none;
        padding: 0;
        margin: 0;
    }

    .book-item {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 0.5rem 0;
        border-bottom: 1px dashed var(--gray);
    }

    .book-item:hover {
        background-color: var(--bg-dark);
    }

    .book-info {
        display: flex;
        gap: 1rem;
    }

    .title {
        font-weight: bold;
        color: var(--blue);
    }

    .author {
        color: var(--gray);
    }

    .book-meta {
        display: flex;
        gap: 1rem;
    }

    .badge {
        font-size: 0.9rem;
    }

    .file-type {
        color: var(--purple);
    }

    .read-status {
        color: var(--aqua);
    }
</style>
