export type SearchIndex = {
    namespace_map: string[];
    feature_map: string[];
    namespaces: {
        [key: string]: [
            {
                name: string;
                features: number[];
            }
        ];
    };
};

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

export class Search {
    private _searchIndex: SearchIndex | undefined;

    async InitializeAsync(branch: string): Promise<boolean> {
        this._searchIndex = {
            feature_map: [],
            namespace_map: [],
            namespaces: {},
        };

        const response = await fetch(
            `https://raw.githubusercontent.com/microsoft/windows-rs/${branch}/crates/libs/windows/features.json`
        );
        if (response.ok) {
            this._searchIndex = await (response.json() as Promise<SearchIndex>);
        }
        return response.ok;
    }

    For(query: SearchQuery): SearchResultSummary {
        const summary: SearchResultSummary = {
            truncated: false,
            results: [],
        };

        if (this._searchIndex === undefined) {
            throw new Error('Search index has not been initialized.');
        }

        for (const namespace in this._searchIndex.namespaces) {
            for (const item of this._searchIndex.namespaces[namespace]) {
                if (summary.results.length >= 50) {
                    summary.truncated = true;
                    break;
                }

                const full_namespace = this._searchIndex.namespace_map[
                    Number.parseInt(namespace)
                ]
                    .replace('Windows.', '')
                    .replace(/\./g, '::');

                const full_name = full_namespace + '::' + item.name;
                let hit = false;
                try {
                    hit = new RegExp(query, 'i').test(full_name);
                } catch {
                    hit = full_name.includes(query);
                }

                if (hit) {
                    let features: string[] = item.features.map(
                        (idx) => this._searchIndex!.feature_map[idx]
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

        return summary;
    }
}
