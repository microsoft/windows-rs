/// <reference lib="webworker" />

import { ISearch, SearchFactory, SearchResult } from './search';

declare let self: ServiceWorkerGlobalScope;

export interface IWorkerMessage {
    name: string;
}

export interface IInitializeMessage extends IWorkerMessage {
    branch: string;
}

export interface ISearchMessage extends IWorkerMessage {
    query: string;
}

export interface ISearchResultMessage extends IWorkerMessage {
    results: SearchResult[];
    truncated: boolean;
}

export interface IInitializeResultMessage extends IWorkerMessage {
    result: boolean;
}

let search: ISearch | undefined = undefined;

self.onmessage = (ev: ExtendableMessageEvent) => {
    const msg: IWorkerMessage = ev.data;
    switch (msg.name) {
        case 'initialize': {
            const initMessage: IInitializeMessage = ev.data;
            SearchFactory.Create(initMessage.branch).then((object) => {
                search = object;
                postMessage({
                    name: 'initializeResult',
                    result: true,
                } as IInitializeResultMessage);
            });
            break;
        }
        default: {
            if (search) {
                const searchMessage: ISearchMessage = ev.data;
                const summary = search.For(searchMessage.query);
                postMessage({
                    name: 'searchResult',
                    results: summary.results,
                    truncated: summary.truncated,
                } as ISearchResultMessage);
            }
        }
    }
};
