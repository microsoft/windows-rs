import { SearchV1 } from './search_v1';
import { SearchV2 } from './search_v2';

export type SearchResultSummary = {
    truncated: boolean;
    results: SearchResult[];
};

export type SearchResult = {
    name: string;
    namespace: string;
    features: string[];
};

export type SearchQuery = string;

export interface ISearch {
    For(query: SearchQuery): SearchResultSummary;
}

export class SearchFactory {
    static async Create(branch: string): Promise<ISearch> {
        const url =
            branch == 'local'
                ? '/static/features.json'
                : `https://raw.githubusercontent.com/microsoft/windows-rs/${branch}/crates/libs/windows/features.json`;

        const response = await fetch(url);
        if (!response.ok) {
            throw new Error('Failed to fetch search index');
        }

        const data = await response.json();
        return 'version' in data && data.version === 2 ? new SearchV2(data) : new SearchV1(data);
    }
}
