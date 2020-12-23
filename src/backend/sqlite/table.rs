use super::*;

impl TableBuilder for SqliteQueryBuilder {
    fn prepare_table_create_statement(&mut self, create: &TableCreateStatement, sql: &mut dyn FmtWrite) {
        write!(sql, "CREATE TABLE ").unwrap();

        if create.create_if_not_exists {
            write!(sql, "IF NOT EXISTS ").unwrap();
        }

        if let Some(table) = &create.table {
            table.prepare(sql, '`');
        }

        write!(sql, " ( ").unwrap();
        let mut count = 0;

        for column_def in create.columns.iter() {
            if count > 0 {
                write!(sql, ", ").unwrap();
            }
            self.prepare_column_def(column_def, sql);
            count += 1;
        }

        for foreign_key in create.foreign_keys.iter() {
            if count > 0 {
                write!(sql, ", ").unwrap();
            }
            self.prepare_foreign_key_create_statement(foreign_key, sql);
            count += 1;
        }

        write!(sql, " )").unwrap();

        for table_opt in create.options.iter() {
            write!(sql, " ").unwrap();
            self.prepare_table_opt(table_opt, sql);
        }
    }

    fn prepare_column_def(&mut self, column_def: &ColumnDef, sql: &mut dyn FmtWrite) {
        column_def.name.prepare(sql, '`');

        if let Some(column_type) = &column_def.types {
            write!(sql, " ").unwrap();
            self.prepare_column_type(&column_type, sql);
        }

        let mut is_primary_key = false;
        let mut is_auto_increment = false;

        for column_spec in column_def.spec.iter() {
            if let ColumnSpec::PrimaryKey = column_spec {
                is_primary_key = true;
                continue
            }
            if let ColumnSpec::AutoIncrement = column_spec {
                is_auto_increment = true;
                continue
            }
            write!(sql, " ").unwrap();
            self.prepare_column_spec(column_spec, sql);
        };

        if is_primary_key {
            write!(sql, " ").unwrap();
            self.prepare_column_spec(&ColumnSpec::PrimaryKey, sql);
        }
        if is_auto_increment {
            write!(sql, " ").unwrap();
            self.prepare_column_spec(&ColumnSpec::AutoIncrement, sql);
        }
    }

    fn prepare_column_type(&mut self, column_type: &ColumnType, sql: &mut dyn FmtWrite) {
        write!(sql, "{}", match column_type {
            ColumnType::Char(length) => format!("text({})", length),
            ColumnType::CharDefault => "text".into(),
            ColumnType::String(length) => format!("text({})", length),
            ColumnType::StringDefault => "text".into(),
            ColumnType::Text => "text".into(),
            ColumnType::TinyInteger(length) => format!("integer({})", length),
            ColumnType::TinyIntegerDefault => "integer".into(),
            ColumnType::SmallInteger(length) => format!("integer({})", length),
            ColumnType::SmallIntegerDefault => "integer".into(),
            ColumnType::Integer(length) => format!("integer({})", length),
            ColumnType::IntegerDefault => "integer".into(),
            ColumnType::BigInteger(length) => format!("integer({})", length),
            ColumnType::BigIntegerDefault => "integer".into(),
            ColumnType::Float(precision) => format!("real({})", precision),
            ColumnType::FloatDefault => "real".into(),
            ColumnType::Double(precision) => format!("real({})", precision),
            ColumnType::DoubleDefault => "real".into(),
            ColumnType::Decimal(precision, scale) => format!("real({}, {})", precision, scale),
            ColumnType::DecimalDefault => "real".into(),
            ColumnType::DateTime(precision) => format!("text({})", precision),
            ColumnType::DateTimeDefault => "text".into(),
            ColumnType::Timestamp(precision) => format!("text({})", precision),
            ColumnType::TimestampDefault => "text".into(),
            ColumnType::Time(precision) => format!("text({})", precision),
            ColumnType::TimeDefault => "text".into(),
            ColumnType::Date => "text".into(),
            ColumnType::Binary(length) => format!("binary({})", length),
            ColumnType::BinaryDefault => "binary".into(),
            ColumnType::Boolean => "integer".into(),
            ColumnType::Money(precision, scale) => format!("integer({}, {})", precision, scale),
            ColumnType::MoneyDefault => "integer".into(),
            ColumnType::Json => "text".into(),
        }).unwrap()
    }

    fn prepare_column_spec(&mut self, column_spec: &ColumnSpec, sql: &mut dyn FmtWrite) {
        write!(sql, "{}", match column_spec {
            ColumnSpec::Null => "NULL".into(),
            ColumnSpec::NotNull => "NOT NULL".into(),
            ColumnSpec::Default(value) => format!("DEFAULT {}", value_to_string(value)),
            ColumnSpec::AutoIncrement => "AUTOINCREMENT".into(),
            ColumnSpec::UniqueKey => "UNIQUE".into(),
            ColumnSpec::PrimaryKey => "PRIMARY KEY".into(),
        }).unwrap()
    }

    fn prepare_table_opt(&mut self, table_opt: &TableOpt, sql: &mut dyn FmtWrite) {
        write!(sql, "{}", match table_opt {
            TableOpt::Engine(s) => format!("ENGINE={}", s),
            TableOpt::Collate(s) => format!("COLLATE={}", s),
            TableOpt::CharacterSet(s) => format!("DEFAULT CHARSET={}", s),
        }).unwrap()
    }

    fn prepare_table_partition(&mut self, _table_partition: &TablePartition, _sql: &mut dyn FmtWrite) {

    }

    fn prepare_table_drop_statement(&mut self, drop: &TableDropStatement, sql: &mut dyn FmtWrite) {
        write!(sql, "DROP TABLE ").unwrap();

        if drop.if_exist {
            write!(sql, "IF EXIST ").unwrap();
        }

        drop.tables.iter().fold(true, |first, table| {
            if !first {
                write!(sql, ", ").unwrap();
            }
            table.prepare(sql, '`');
            false
        });

        for drop_opt in drop.options.iter() {
            write!(sql, " ").unwrap();
            self.prepare_table_drop_opt(drop_opt, sql);
        }
    }

    fn prepare_table_drop_opt(&mut self, drop_opt: &TableDropOpt, sql: &mut dyn FmtWrite) {
        write!(sql, "{}", match drop_opt {
            TableDropOpt::Restrict => "RESTRICT",
            TableDropOpt::Cascade => "CASCADE",
        }).unwrap();
    }

    fn prepare_table_truncate_statement(&mut self, truncate: &TableTruncateStatement, sql: &mut dyn FmtWrite) {
        write!(sql, "TRUNCATE TABLE ").unwrap();

        if let Some(table) = &truncate.table {
            table.prepare(sql, '`');
        }
    }

    fn prepare_table_alter_statement(&mut self, alter: &TableAlterStatement, sql: &mut dyn FmtWrite) {
        let alter_option = match &alter.alter_option {
            Some(alter_option) => alter_option,
            None => panic!("No alter option found"),
        };
        write!(sql, "ALTER TABLE ").unwrap();
        if let Some(table) = &alter.table {
            table.prepare(sql, '`');
            write!(sql, " ").unwrap();
        }
        match alter_option {
            TableAlterOption::AddColumn(column_def) => {
                write!(sql, "ADD COLUMN ").unwrap();
                self.prepare_column_def(column_def, sql);
            },
            TableAlterOption::ModifyColumn(_) => {
                panic!("Sqlite not support modifying table column")
            },
            TableAlterOption::RenameColumn(from_name, to_name) => {
                write!(sql, "RENAME COLUMN ").unwrap();
                from_name.prepare(sql, '`');
                write!(sql, " TO ").unwrap();
                to_name.prepare(sql, '`');
            },
            TableAlterOption::DropColumn(_) => {
                panic!("Sqlite not support dropping table column")
            },
        }
    }

    fn prepare_table_rename_statement(&mut self, rename: &TableRenameStatement, sql: &mut dyn FmtWrite) {
        write!(sql, "ALTER TABLE ").unwrap();
        if let Some(from_name) = &rename.from_name {
            from_name.prepare(sql, '`');
        }
        write!(sql, " RENAME TO ").unwrap();
        if let Some(to_name) = &rename.to_name {
            to_name.prepare(sql, '`');
        }
    }
}