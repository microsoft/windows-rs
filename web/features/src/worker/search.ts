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

        const namespaces = this._searchIndex.namespaces;
        for (const namespace in namespaces) {
            for (const item of namespaces[namespace]!) {
                if (summary.results.length >= 50) {
                    summary.truncated = true;
                    break;
                }

                const full_namespace = this._searchIndex.namespace_map[
                    Number.parseInt(namespace)
                ]!
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

        // Sorts results by match position and suffix length.
        // "createprocess" in "CreateProcess" beats "SHCREATEPROCESSINFOW".
        // "createprocess" in "CreateProcessW" beats "CreateProcessWithTokenW".
        summary.results.sort((a, b) => {
            const query_lower = query.toLowerCase();
            const a_lower = a.name.toLowerCase();
            const b_lower = b.name.toLowerCase();
            
            const pos_a = a_lower.indexOf(query_lower);
            const pos_b = b_lower.indexOf(query_lower);
            
            if (pos_a !== pos_b) {
                return pos_a - pos_b;
            }
            
            const suffix_len_a = a.name.length - (pos_a + query.length);
            const suffix_len_b = b.name.length - (pos_b + query.length);
            return suffix_len_a - suffix_len_b;
        });

        return summary;
    }
}
