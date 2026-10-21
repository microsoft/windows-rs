use super::*;

#[test]
fn cleared_font_size_restores_inherited_value() -> Result<()> {
    process::run("typography-inherited", |ui| {
        let header = ui.element("header")?;
        let width = || unsafe {
            let bounds = header.CurrentBoundingRectangle()?;
            Ok(bounds.right - bounds.left)
        };
        let inherited = ui.wait("header has nonzero width", width, |width| *width > 0)?;

        ui.invoke("apply")?;
        let local = ui.wait("local font size shrinks the header", width, |width| {
            *width < inherited * 3 / 4
        })?;
        assert!(local > 0);

        ui.invoke("reset")?;
        ui.wait(
            "clearing the font size restores the inherited value",
            width,
            |width| *width == inherited,
        )?;
        assert!(unsafe {
            ui.client
                .CompareElements(&header, &ui.element("header")?)?
                .as_bool()
        });
        Ok(())
    })
}

#[test]
fn text_block_typography_changes_layout_and_resets_in_place() -> Result<()> {
    process::run("typography", |ui| {
        let ids = ["family", "spacing", "height"];
        let elements = ids
            .iter()
            .map(|id| ui.element(id))
            .collect::<Result<Vec<_>>>()?;
        let measure = || unsafe {
            let family = elements[0].CurrentBoundingRectangle()?;
            let spacing = elements[1].CurrentBoundingRectangle()?;
            let height = elements[2].CurrentBoundingRectangle()?;
            Ok([
                family.right - family.left,
                spacing.right - spacing.left,
                height.bottom - height.top,
            ])
        };
        let baseline = ui.wait("text has nonzero bounds", measure, |sizes| {
            sizes.iter().all(|size| *size > 0)
        })?;

        ui.invoke("apply")?;
        let applied = ui.wait(
            "font, spacing, and line height expand the text",
            measure,
            |sizes| {
                sizes[0] > baseline[0] * 3 / 2 && sizes[1] > baseline[1] && sizes[2] > baseline[2]
            },
        )?;

        ui.invoke("change")?;
        let changed = ui.wait(
            "updated typography changes each retained text block",
            measure,
            |sizes| {
                sizes[0] < applied[0] * 3 / 4
                    && sizes[1] < baseline[1]
                    && sizes[2] > baseline[2]
                    && sizes[2] < applied[2]
            },
        )?;

        ui.invoke("automatic")?;
        ui.wait("zero restores automatic line height", measure, |sizes| {
            *sizes == [changed[0], changed[1], baseline[2]]
        })?;
        ui.invoke("apply")?;
        ui.wait("explicit typography reapplies", measure, |sizes| {
            *sizes == applied
        })?;
        ui.invoke("reset")?;
        ui.wait(
            "omitted properties restore the original layout",
            measure,
            |sizes| *sizes == baseline,
        )?;
        for (id, original) in ids.iter().zip(&elements) {
            assert!(unsafe {
                ui.client
                    .CompareElements(original, &ui.element(id)?)?
                    .as_bool()
            });
        }
        Ok(())
    })
}
