import { SearchQuery, SearchResultSummary } from './search';
import { sortSearchResults } from './search_utils';

export type SearchIndexV1 = {
    namespace_map: string[];
    feature_map: string[];
    namespaces: {
        [key: string]: {
            name: string;
            features: number[];
        }[];
    };
};

export class SearchV1 {
    private _searchIndex: SearchIndexV1;

    constructor(searchIndex: SearchIndexV1) {
        this._searchIndex = searchIndex;
    }

    For(query: SearchQuery): SearchResultSummary {
        const summary: SearchResultSummary = {
            truncated: false,
            results: [],
        };

        if (this._searchIndex === undefined) {
            throw new Error('Search index has not been initialized.');
        }

        const namespaces = this._searchIndex.namespaces;
        for (const namespace in namespaces) {
            for (const item of namespaces[namespace]!) {
                if (summary.results.length >= 50) {
                    summary.truncated = true;
                    break;
                }

                const full_namespace = this._searchIndex.namespace_map[
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
                    let features: string[] = item.features.map(
                        (idx) => this._searchIndex!.feature_map[idx]!
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
