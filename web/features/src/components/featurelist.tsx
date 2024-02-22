import { makeStyles, mergeClasses } from '@fluentui/react-components';

const useStyles = makeStyles({
    list: {
        whiteSpace: 'pre'
    },
});

export interface FeatureListProps {
    className?: string;
    features: string[];
}

export function FeatureList({ features, className }: FeatureListProps) {
    const styles = useStyles();
    return (
        <span className={mergeClasses(className, styles.list)}>
            {features.map((f) => `"${f}"`).join(',\n')}
        </span>
    );
}

export default FeatureList;
