//! @brief solana-features-diff utility functions
use console::{style, StyledObject};
use gadgets_scfs::{ScfsMatrix, ScfsRow, ScfsStatus, SCFS_DESCRIPTION, SCFS_FEATURE_ID};

#[derive(Debug)]
struct FieldFormatter {
    field_name: String,
    cluster_index: usize,
    is_feature_id: bool,
    is_description: bool,
}

impl FieldFormatter {
    fn build_formats(matrix: &ScfsMatrix) -> Vec<FieldFormatter> {
        let mut fmthdr = Vec::<FieldFormatter>::new();
        let mut cluster_pos = 0usize;

        for field in matrix.get_criteria().fields.as_ref().unwrap() {
            if field == &*SCFS_FEATURE_ID {
                fmthdr.push(FieldFormatter {
                    field_name: field.clone(),
                    cluster_index: 0,
                    is_feature_id: true,
                    is_description: false,
                })
            } else if field == &*SCFS_DESCRIPTION {
                fmthdr.push(FieldFormatter {
                    field_name: field.clone(),
                    cluster_index: 0,
                    is_feature_id: false,
                    is_description: true,
                })
            } else {
                fmthdr.push(FieldFormatter {
                    field_name: field.clone(),
                    cluster_index: cluster_pos,
                    is_feature_id: false,
                    is_description: false,
                });
                cluster_pos += 1;
            }
        }
        fmthdr
    }
}

#[derive(Debug)]
struct MatrixStdOut<'a> {
    matrix: &'a ScfsMatrix,
    fmthdr: Vec<FieldFormatter>,
}

impl<'a> MatrixStdOut<'a> {
    fn new(matrix: &'a ScfsMatrix) -> Self {
        let fmt = FieldFormatter::build_formats(matrix);
        Self {
            matrix,
            fmthdr: fmt,
        }
    }
}

fn fill_format_tuple(
    row: &ScfsRow,
    field_fmt: &Vec<FieldFormatter>,
) -> (
    String,
    StyledObject<String>,
    StyledObject<String>,
    StyledObject<String>,
    StyledObject<String>,
    String,
) {
    let blank = "".to_string();
    let pk = row.key().to_string();
    let mut local_state = style(blank.clone()).bg(console::Color::Black);
    let mut dev_state = style(blank.clone()).bg(console::Color::Black);
    let mut test_state = style(blank.clone()).bg(console::Color::Black);
    let mut main_state = style(blank.clone()).bg(console::Color::Black);
    let mut desc = "".to_string();
    fn fill_status(status: &ScfsStatus) -> StyledObject<String> {
        let yes = " ".to_string();
        let no = "  ".to_string();
        match status {
            ScfsStatus::Inactive => style(no).bg(console::Color::Red),
            ScfsStatus::Pending => style(no).bg(console::Color::Yellow),
            ScfsStatus::Active(_) => style(yes).bg(console::Color::Green),
        }
    }
    let row_status = row.status();
    for ff in field_fmt {
        match ff.field_name.as_str() {
            "description" => {
                if let Some(description) = row.desc() {
                    desc = description.clone();
                }
            }
            "local" => {
                if ff.cluster_index < row_status.len() {
                    local_state = fill_status(&row_status[ff.cluster_index]);
                }
            }
            "devnet" => {
                if ff.cluster_index < row_status.len() {
                    dev_state = fill_status(&row_status[ff.cluster_index]);
                }
            }
            "testnet" => {
                if ff.cluster_index < row_status.len() {
                    test_state = fill_status(&row_status[ff.cluster_index]);
                }
            }
            "mainnet" => {
                if ff.cluster_index < row_status.len() {
                    main_state = fill_status(&row_status[ff.cluster_index]);
                }
            }
            _ => {}
        }
    }
    (pk, local_state, dev_state, test_state, main_state, desc)
}
impl std::fmt::Display for MatrixStdOut<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Write header
        for field in &self.fmthdr {
            if field.is_feature_id {
                write!(f, "{}", style(format!("{:<44} ", field.field_name)).bold())?;
            } else if field.is_description {
                write!(f, "{}", style(format!("| {:<95}", field.field_name)).bold())?;
            } else {
                write!(f, "{}", style(format!("| {:<8} ", field.field_name)).bold())?;
            }
        }
        writeln!(f)?;
        // Underline separator
        writeln!(
            f,
            "{}",
            style(format!(
                "{:-<44} | {:-^8} | {:-^8} | {:-^8} | {:-^8} | {:-<95}",
                "", "", "", "", "", ""
            ))
        )?;
        // Data fields
        for row in self.matrix.get_result_rows() {
            let (pk, local, dev, test, main, desc) = fill_format_tuple(row, &self.fmthdr);
            writeln!(
                f,
                "{:<44} | {:^8} | {:^8} | {:^8} | {:^8} | {:<95}",
                pk, local, dev, test, main, desc,
            )?;
            // break;
        }

        Ok(())
    }
}

pub fn write_matrix_stdio(matrix: &ScfsMatrix) {
    let mxout = MatrixStdOut::new(matrix);
    println!("{}", mxout);
}

#[cfg(test)]
mod tests {
    use crate::utils::write_matrix_stdio;
    use gadgets_scfs::{ScfsCriteria, ScfsMatrix, SCFS_DEVNET, SCFS_LOCAL};

    #[test]
    fn test_header_gen_pass() {
        let mut cluster_vec = Vec::<String>::new();
        cluster_vec.push(SCFS_LOCAL.to_string());
        cluster_vec.push(SCFS_DEVNET.to_string());
        let mut my_matrix = ScfsMatrix::new(Some(ScfsCriteria {
            clusters: Some(cluster_vec),
            ..Default::default()
        }))
        .unwrap();
        assert!(my_matrix.run().is_ok());
        write_matrix_stdio(&my_matrix);
    }
    #[test]
    fn formatting_test() {
        let mut fs = [""; 5];
        fs[2] = "yes";
        println!("{:?}", fs);
        let x = format!("{}", 1);
        println!("{}", x)
    }
}
