use std::{
    convert::identity,
    env::current_dir,
    fs::{read_dir, read_to_string},
    path::{Path, PathBuf},
};

use anyhow::{Result, anyhow};

use crate::file::{New, Parsed, SchemaFile, Validated};

const SCHEMA_PATHS: [&str; 1] = ["prisma/schema.prisma"];

///
///
///
#[derive(Debug)]
pub struct SchemaContext<SC = New, FC = New>
where
    SC: Clone,
    FC: Clone,
{
    /// The root directory containing schema files.
    pub root_dir: PathBuf,
    pub files: Vec<SchemaFile<FC>>,
    pub context: SC,
    // Real {
    // },
    // Virtual {
    //     content: String,
    // },
}

impl<T1, T2> SchemaContext<T1, T2>
where
    T1: Clone,
    T2: Clone,
{
    pub fn list_migrations(&self) {
        let migrations_dir = self.root_dir.join("migrations");

        // // If directory doesn't exist, return an empty array
        if !migrations_dir.try_exists().is_ok_and(identity) {
            panic!("Migration directory does not exist");
            /*
            return {
                        baseDir,
                        lockfile,
                        migrationDirectories: [],
                        shadowDbInitScript,
                      }
            */
        }

        // entries = await fs.readdir(migrationsDirectoryPath, { withFileTypes: true, recursive: false }).catch((_) => [])
        let mut entries = read_dir(migrations_dir)
            .unwrap()
            .into_iter()
            .map(|entry| entry.unwrap())
            .filter(|entry| entry.file_type().unwrap().is_dir())
            .map(|entry| {
                let migration_file = entry.path().join("migration.sql");

                let content = if migration_file.try_exists().is_ok_and(identity) {
                    read_to_string(&migration_file).unwrap()
                } else {
                    return None;
                };

                Some((migration_file, content))
            })
            .filter(|it| it.is_some())
            .map(|it| it.unwrap())
            .collect::<Vec<_>>();

        entries.sort_by(|a, b| a.0.cmp(&b.0));

        println!("{:#?}", entries);

        // entries

        /*
        return {
                        baseDir,
                        lockfile,
                        migrationDirectories: sortedMigrations,
                        shadowDbInitScript,
                      }
        */
    }
}

impl SchemaContext<New, New> {
    pub fn load(cwd: Option<PathBuf>) -> Result<Self> {
        let cwd = cwd.unwrap_or(current_dir()?);

        for path in SCHEMA_PATHS {
            let full_path = cwd.join(path);

            if full_path.try_exists().is_ok_and(identity) {
                if full_path.is_dir() {
                    return Err(anyhow!(
                        "Path {:?} was expected to be a file but was a directory.",
                        full_path
                    ));
                }

                return Ok(Self {
                    root_dir: full_path.parent().unwrap().to_owned(),
                    files: Self::load_files(&full_path.parent().unwrap()).unwrap_or(vec![]),
                    context: New,
                });
            }
        }

        Err(anyhow!("Could not find schema.prisma file"))
    }

    pub fn files(&self) -> &[SchemaFile<New>] {
        &self.files
    }

    fn load_files<P: AsRef<Path>>(path: P) -> Option<Vec<SchemaFile>> {
        let path = path.as_ref();

        assert!(path.is_dir());

        let mut files: Vec<SchemaFile<New>> = vec![];

        for file in path.read_dir().unwrap() {
            let file = file.unwrap();
            let meta = file.metadata().unwrap();

            if meta.is_file() && file.path().extension().is_some_and(|it| it == "prisma") {
                files.push(SchemaFile::new(file.path().canonicalize().unwrap()));
            }

            if meta.is_dir()
                && let Some(inner) = Self::load_files(file.path())
            {
                files.extend(inner);
            }
        }

        if files.is_empty() { None } else { Some(files) }
    }

    pub fn parse<T: SchemaParser>(self) -> SchemaContext<Parsed<T::Context>, Parsed<T::File>> {
        T::parse(self)
    }
}

impl<T: Clone> SchemaContext<Parsed<T>> {}

impl<T: Clone> SchemaContext<Validated<T>> {}

pub trait SchemaParser {
    type Context: Clone;
    type File: Clone;

    fn parse(schema: SchemaContext<New, New>) -> SchemaContext<Parsed<Self::Context>, Parsed<Self::File>>;
}

pub trait SchemaValidator {
    type Context: Clone;
    type File: Clone;

    fn validate(&self) -> SchemaContext<Validated<Self::Context>, Validated<Self::File>>;
}

/*
  /**
   * All loaded schema files and their paths.
   */
  schemaFiles: LoadedFile[]
  /**
   * The root directory of the schema files.
   * Either set explicitly from a schema folder based config or the parent directory of the schema.prisma file.
   */
  schemaRootDir: string
  /**
   * The directory of the schema.prisma file that contains the datasource block.
   * Some relative paths like SQLite paths or SSL file paths are resolved relative to it.
   * TODO(prisma7): consider whether relative paths should be resolved relative to `prisma.config.ts` instead.
   */
  primaryDatasourceDirectory: string
  /**
   * The path that shall be printed in user facing logs messages informing them from where the schema was loaded.
   */
  loadedFromPathForLogMessages: string
  /**
   * The datasource extracted from the Prisma schema. So far we only support a single datasource block.
   */
  primaryDatasource: DataSource | undefined
  /**
   * Warnings that were raised during Prisma schema parsing.
   */
  warnings: string[] | []
  /**
   * The datasources extracted from the Prisma schema. Prefer to use primaryDatasource for most cases.
   */
  datasources: DataSource[] | []
  /**
   * The generators extracted from the Prisma schema.
   */
  generators: GeneratorConfig[] | []
  /**
   * @deprecated Only used during the refactoring for backwards compatibility. Use `schemaFiles` instead or determine needed file paths otherwise.
   */
  schemaPath: string

*/
