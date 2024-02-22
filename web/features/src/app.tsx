import React from 'react';
import { SearchResult } from './worker/search';
import {
    makeStyles,
    mergeClasses,
    shorthands,
    Input,
    DataGridHeaderCell,
    DataGridBody,
    DataGridCell,
    DataGridHeader,
    DataGridRow,
    DataGrid,
    TableColumnDefinition,
    TableCellLayout,
    InputOnChangeData,
    createTableColumn,
    InfoLabel,
    Dropdown,
    Option,
    OptionOnSelectData,
    SelectionEvents
} from '@fluentui/react-components';
import Path from './components/path';
import FeatureList from './components/featurelist';
import { IWorkerMessage, IInitializeMessage, IInitializeResultMessage, ISearchMessage, ISearchResultMessage } from './worker/worker';
import { useNavigate, useParams } from 'react-router-dom';

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
        flexGrow: 1
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
    const [searchResults, setSearchResults] = React.useState<SearchResult[]>([]);
    const [resultsTruncated, setResultsTruncated] = React.useState(false);
    const [searchTimeoutId, setSearchTimeoutId] = React.useState(0);
    const [searchReady, setSearchReady] = React.useState(false);
    const [worker, setWorker] = React.useState<Worker | null>(null);
    const [query, setQuery] = React.useState('');

    const params = useParams();
    const branches = process.env.REACT_APP_BRANCHES!.split(',');
    const defaultBranch = branches.includes(params.branch!) ? params.branch : branches[0];
    const [branch, setBranch] = React.useState(defaultBranch);

    const styles = useStyles();
    const onMessage = function ({ data }: MessageEvent<IWorkerMessage>) {
        switch (data.name) {
            case 'initializeResult':
            {
                const msg = data as IInitializeResultMessage;
                setSearchReady(msg.result);
                break;
            }
            case 'searchResult':
            {
                const msg = data as ISearchResultMessage;
                setSearchResults(msg.results);
                setResultsTruncated(msg.truncated);
                break;
            }
        }
    };

    React.useEffect(() => {
        const worker = new Worker(
            new URL('./worker/worker.ts', import.meta.url)
        );
        worker.addEventListener('message', onMessage);
        worker.postMessage({
            name: 'initialize',
            branch
        } as IInitializeMessage);
        setWorker(worker);

        return () => {
            worker.removeEventListener('message', onMessage);
            worker.terminate();
        };
    }, [branch]);

    React.useEffect(() => {
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
    }, [query, worker]);

    const onChangeQuery = (
        ev: React.ChangeEvent<HTMLInputElement>,
        data: InputOnChangeData
    ) => {
        setQuery(data.value);
        setResultsTruncated(false);
    };

    const navigate = useNavigate();
    const onBranchSelected = (
        ev: SelectionEvents,
        data: OptionOnSelectData
    ) => {
        const branch = data.optionValue ?? defaultBranch;
        setBranch(branch);
        navigate(`/${branch}`, { replace: true });
    };

    return (
        <>
            <div className={styles.root}>
                <div className={styles.searchContainer}>
                    <Input
                        className={styles.searchInput}
                        size='large'
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
                </div>

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
