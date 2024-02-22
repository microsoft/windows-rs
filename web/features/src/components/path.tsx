import { makeStyles } from '@fluentui/react-components';
import { tokens } from '@fluentui/react-theme';

const useStyles = makeStyles({
    namespace: {
        overflowWrap: 'anywhere',
    },
    name: {
        color: tokens.colorBrandForeground2,
    },
});

export interface PathProps {
    namespace: string;
    name: string;
}

export function Path({ name, namespace }: PathProps) {
    const styles = useStyles();
    return (
        <span className={styles.namespace}>
            {namespace}::<span className={styles.name}>{name}</span>
        </span>
    );
}

export default Path;
