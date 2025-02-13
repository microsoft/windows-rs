import {
    Button,
    createTableColumn,
    DataGrid,
    DataGridBody,
    DataGridCell,
    DataGridHeader,
    DataGridHeaderCell,
    DataGridRow,
    Dropdown,
    InfoLabel,
    Input,
    InputOnChangeData,
    makeStyles,
    mergeClasses,
    Option,
    OptionOnSelectData,
    SelectionEvents,
    shorthands,
    TableCellLayout,
    TableColumnDefinition
} from '@fluentui/react-components';
import { WeatherMoonRegular, WeatherSunnyRegular } from '@fluentui/react-icons';
import React, { useCallback, useEffect, useState } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import FeatureList from './components/featurelist';
import Path from './components/path';
import { SearchResult } from './worker/search';
import { IInitializeMessage, IInitializeResultMessage, ISearchMessage, ISearchResultMessage, IWorkerMessage } from './worker/worker';

const useStyles = makeStyles({
    root: {
        '& h1': { fontSize: '2rem' },
        '& h2': { fontSize: '1.5rem' },
        '& h3': { fontSize: '1.0rem' },
        fontSize: '16px',
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        maxWidth: '90vw',
        ...shorthands.margin('1em', 'auto'),
    },
    grid: {
        width: '100%'
    },
    desktopOnly: {
        '@media (max-width: 768px)': {
            display: 'none',
        },
    },
    mobileGrid: {
        '@media (min-width: 769px)': {
            display: 'none',
        },
        display: 'flex',
        flexDirection: 'column',
        width: '100%',
        ...shorthands.gap('1em'),
    },
    featureList: {
        textAlign: 'right',
    },
    header: {
        alignSelf: 'flex-start'
    },
    justifyContentEnd: {
        justifyContent: 'end',
    },
    hidden: {
        visibility: 'hidden',
    },
    visible: {
        visibility: 'visible',
    },
    searchContainer: {
        display: 'flex',
        flexWrap: 'wrap',
        width: '100%',
        ...shorthands.gap('1em')
    },
    searchInput: {
        flexGrow: 5
    },
    searchBranch: {
        flexGrow: 1,
        minWidth: '200px'
    },
    mobileRow: {
        display: 'grid',
        gap: '20px',
        paddingBottom: '16px',
        flexDirection: 'column',
        borderBottomWidth: '1px',
        borderBottomStyle: 'solid',
        borderBottomColor: 'var(--colorNeutralStroke1)',
        ...shorthands.gap('0.5em'),
    },
    labelValuePair: {
        display: 'grid',
        gridTemplateRows: 'auto auto',
        gap: '4px',
    },
    label: {
        fontWeight: 'bold',
        fontSize: '.75em'
    }
});

const columns: TableColumnDefinition<Renderable<SearchResult>>[] = [
    createTableColumn({
        columnId: 'name',
        compare: (a, b) => a.name.localeCompare(b.name),
        renderCell: (result) => {
            return (
                <TableCellLayout>
                    <Path namespace={result.namespace} name={result.name} />
                </TableCellLayout>
            );
        },
        renderHeaderCell: () => {
            return <TableCellLayout><h3>name</h3></TableCellLayout>;
        },
    }),
    createTableColumn({
        columnId: 'features',
        compare: (a, b) =>
            JSON.stringify(a.features).localeCompare(
                JSON.stringify(b.features)
            ),
        renderCell: (result) => {
            return (
                <TableCellLayout
                    className={mergeClasses(
                        result.classes.featureList,
                        result.classes.justifyContentEnd
                    )}
                >
                    <FeatureList features={result.features} />
                </TableCellLayout>
            );
        },
        renderHeaderCell: (data) => {
            const styles = data as ReturnType<typeof useStyles>;
            return (
                <TableCellLayout className={styles.justifyContentEnd}>
                    <h3>features</h3>
                </TableCellLayout>
            );
        },
    }),
];

type Renderable<TItem> = TItem & { classes: ReturnType<typeof useStyles> };

function App() {
    // State

    const params = useParams();
    const styles = useStyles();
    const navigate = useNavigate();

    const [worker, setWorker] = useState<Worker | null>(null);
    const [searchResults, setSearchResults] = useState<SearchResult[]>([]);
    const [resultsTruncated, setResultsTruncated] = useState(false);
    const [searchTimeoutId, setSearchTimeoutId] = useState(0);
    const [searchReady, setSearchReady] = useState(false);
    const [query, setQuery] = useState(params.query! ?? '');
    const [savedTheme, setSavedTheme] = useState(localStorage.getItem('theme') ?? 'light');

    const branches = process.env.REACT_APP_BRANCHES!.split(',');
    if (process.env.REACT_APP_DEBUG) {
        branches.push('local');
    }

    const defaultBranch = branches.includes(params.branch!) ? params.branch : branches[0];
    const [branch, setBranch] = useState(defaultBranch);

    // Effects and Hooks

    const onMessage = useCallback(({ data }: MessageEvent<IWorkerMessage>) => {
        if (data.name === 'initializeResult') {
            const msg = data as IInitializeResultMessage;
            setSearchReady(msg.result);
            return;
        }

        if (data.name === 'searchResult') {
            const msg = data as ISearchResultMessage;
            setSearchResults(msg.results);
            setResultsTruncated(msg.truncated);
            return;
        }
    }, []);

    useEffect(() => {
        const newWorker = new Worker(new URL('./worker/worker.ts', import.meta.url));
        newWorker.addEventListener('message', onMessage);
        newWorker.postMessage({
            name: 'initialize',
            branch
        } as IInitializeMessage);

        setWorker(newWorker);

        return () => {
            newWorker.removeEventListener('message', onMessage);
            newWorker.terminate();
            setWorker(null);
            setSearchReady(false);
        };
    }, [branch, onMessage]);

    useEffect(() => {
        if (!searchReady) return;

        if (query.length > 0) {
            if (searchTimeoutId) {
                clearTimeout(searchTimeoutId);
            }

            setSearchTimeoutId(
                window.setTimeout(() => {
                    worker!.postMessage({
                        name: 'search',
                        query: query
                    } as ISearchMessage);
                }, 10)
            );
        } else {
            setSearchResults([]);
        }
    // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [query, worker, searchReady]);

    // Event handlers

    const onChangeQuery = (
        ev: React.ChangeEvent<HTMLInputElement>,
        data: InputOnChangeData
    ) => {
        setQuery(data.value);
        setResultsTruncated(false);

        if (data.value.length > 0) {
            navigate(`/${branch}/search/${data.value}`, { replace: true });
        } else {
            navigate(`/${branch}`, { replace: true });
        }
    };

    const onThemeChange = () => {
        const newTheme = savedTheme === 'light' ? 'dark' : 'light';
        localStorage.setItem('theme', newTheme);
        setSavedTheme(newTheme);
        window.location.reload();
    };

    const onBranchSelected = (
        ev: SelectionEvents,
        data: OptionOnSelectData
    ) => {
        const branch = data.optionValue ?? defaultBranch;
        setBranch(branch);

        if (query.length > 0) {
            navigate(`/${branch}/search/${query}`, { replace: true });
        } else {
            navigate(`/${branch}`, { replace: true });
        }
    };

    return (
        <>
            <div className={styles.root}>
                <div className={styles.searchContainer}>
                    <Input
                        className={styles.searchInput}
                        size='large'
                        value={query}
                        onChange={onChangeQuery}
                        spellCheck='false'
                        autoComplete='off'
                        autoCorrect='off'
                        autoCapitalize='off'
                        disabled={!searchReady}
                        placeholder={searchReady ? '' : 'Loading index...'}
                        contentAfter={
                            (
                                <>
                                    <InfoLabel
                                        size='large'
                                        info={
                                            <div>
                                                <p>
                                                    You can search for types using words:<br />
                                                    <code>CreateFile</code>
                                                </p>
                                                <p>
                                                    Or regular expressions:<br />
                                                    <code>::CreateFile2.*?</code>
                                                </p>
                                            </div>
                                        }
                                    >
                                    </InfoLabel>
                                </>
                            )
                        }
                    />
                    <Dropdown size='large' className={styles.searchBranch} value={branch} onOptionSelect={onBranchSelected}>
                        {
                            branches.map((branch) => (
                                <Option key={branch}>
                                    {branch}
                                </Option>
                            ))
                        }
                    </Dropdown>
                    <Button
                        icon={savedTheme === 'dark' ? <WeatherSunnyRegular /> : <WeatherMoonRegular />}
                        onClick={onThemeChange}
                        appearance="transparent"
                    />
                </div>

                <div className={mergeClasses(styles.grid, styles.desktopOnly)}>
                    <DataGrid
                        items={searchResults}
                        columns={columns}
                        focusMode='none'
                        size='medium'
                        className={styles.grid}
                    >
                        <DataGridHeader>
                            <h2>
                                Results ({searchResults.length}
                                {resultsTruncated ? '+' : ''})
                            </h2>
                            <DataGridRow>
                                {(col) => {
                                    return (
                                        <DataGridHeaderCell key={col.columnId}>
                                            {col.renderHeaderCell(styles)}
                                        </DataGridHeaderCell>
                                    );
                                }}
                            </DataGridRow>
                        </DataGridHeader>
                        <DataGridBody<SearchResult>>
                            {({ item, rowId }) => {
                                return (
                                    <DataGridRow
                                        key={rowId}
                                        style={{
                                            paddingTop: '1em',
                                            paddingBottom: '1em',
                                        }}
                                    >
                                        {(col) => (
                                            <DataGridCell>
                                                {col.renderCell({
                                                    ...item,
                                                    classes: styles,
                                                })}
                                            </DataGridCell>
                                        )}
                                    </DataGridRow>
                                );
                            }}
                        </DataGridBody>
                    </DataGrid>
                </div>

                <div className={styles.mobileGrid}>
                    <h2>
                        Results ({searchResults.length}
                        {resultsTruncated ? '+' : ''})
                    </h2>
                    {searchResults.map((item, i) => (
                        <div key={i} className={styles.mobileRow}>
                            <div className={styles.labelValuePair}>
                                <span className={styles.label}>name</span>
                                <Path namespace={item.namespace} name={item.name} />
                            </div>
                            <div className={styles.labelValuePair}>
                                <span className={styles.label}>features</span>
                                <FeatureList features={item.features} />
                            </div>
                        </div>
                    ))}
                </div>

                <p
                    className={
                        resultsTruncated ? styles.visible : styles.hidden
                    }
                >
                    (Only the first 50 results are shown.)
                </p>
            </div>
        </>
    );
}

export default App;
