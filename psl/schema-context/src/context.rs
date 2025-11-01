use crate::file::{New, Parsed, Resolved, SchemaFile, Validated};

///
///
///
pub enum SchemaContext<T = New> {
    Real { 
        /// The root directory containing schema files.
        root_dir: String, 
        files: Vec<SchemaFile<T>>,
        context: T,
    },
    Virtual { content: String },
}

impl SchemaContext<New> {}

impl SchemaContext<Resolved> {}

impl SchemaContext<Parsed> {}

impl SchemaContext<Validated> {}



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