import { SearchResult } from './search';

// Sorts results by match position and suffix length.
// "createprocess" in "CreateProcess" beats "SHCREATEPROCESSINFOW".
// "createprocess" in "CreateProcessW" beats "CreateProcessWithTokenW".
export function sortSearchResults(results: SearchResult[], query: string) {
    return results.sort((a, b) => {
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
}
