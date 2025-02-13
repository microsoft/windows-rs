import { SearchQuery, SearchResultSummary } from './search';
import { sortSearchResults } from './search_utils';

export type SearchIndexV2 = {
    version: number;
    namespaces: string[];
    items: {
        [key: string]: Array<{ name: string; features: number[] }>;
    };
};

export type CompressedSearchIndexV2 = {
    version: number;
    namespaces: string[];
    items: {
        [key: string]: Array<{ n: string; f: number[] }>;
    };
};

export class SearchV2 {
    private _searchIndex: SearchIndexV2 | undefined;

    constructor(json: CompressedSearchIndexV2) {
        this._searchIndex = {
            version: json.version,
            namespaces: json.namespaces,
            items: Object.fromEntries(
                Object.entries(json.items).map(([key, value]) => [
                    key,
                    (value as Array<{ n: string; f: number[] }>).map((item) => ({
                        name: item.n.replace(/\./g, '::'),
                        features: item.f,
                    })),
                ])
            ),
        } as SearchIndexV2;
    }

    For(query: SearchQuery): SearchResultSummary {
        const summary: SearchResultSummary = {
            truncated: false,
            results: [],
        };

        if (this._searchIndex === undefined) {
            throw new Error('Search index has not been initialized.');
        }

        for (const [namespace, items] of Object.entries(this._searchIndex.items)) {
            for (const item of items) {
                if (summary.results.length >= 50) {
                    summary.truncated = true;
                    break;
                }

                const full_namespace = this._searchIndex.namespaces[
                    Number.parseInt(namespace)
                ]!.replace('Windows.', '').replace(/\./g, '::');

                const full_name = full_namespace + '::' + item.name;
                let hit = false;
                try {
                    hit = new RegExp(query, 'i').test(full_name);
                } catch {
                    hit = full_name.includes(query);
                }

                if (hit) {
                    let features: string[] = item.features.map((idx) =>
                        this._searchIndex!.namespaces[idx]!.replace('Windows.', '').replace(
                            /\./g,
                            '_'
                        )
                    );

                    features = features.filter(
                        (f) => f !== 'Foundation' && f !== 'Win32_Foundation'
                    );

                    summary.results.push({
                        name: item.name,
                        namespace: full_namespace,
                        features: features,
                    });
                }
            }
        }

        summary.results = sortSearchResults(summary.results, query);
        return summary;
    }
}
