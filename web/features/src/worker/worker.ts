/// <reference lib="webworker" />

import { Search, SearchResult } from './search';

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

const search = new Search();

self.onmessage = (ev: ExtendableMessageEvent) => {
    const msg: IWorkerMessage = ev.data;
    switch (msg.name) {
        case 'initialize': {
            const initMessage: IInitializeMessage = ev.data;
            search.InitializeAsync(initMessage.branch).then(result => {
                postMessage({
                    name: 'initializeResult',
                    result: result,
                } as IInitializeResultMessage);
            });
            break;
        }
        default: {
            const searchMessage: ISearchMessage = ev.data;
            const summary = search.For(searchMessage.query);
            postMessage({
                name: 'searchResult',
                results: summary.results,
                truncated: summary.truncated,
            } as ISearchResultMessage);
        }
    }
};
