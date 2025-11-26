use psl::Datasource;

/*
// check if we can connect to the database
// if true: return true
// if false: throw error
export async function ensureCanConnectToDatabase(datasource: DataSource | undefined): Promise<Boolean | Error> {
  if (!datasource) {
    throw new Error(`A datasource block is missing in the Prisma schema file.`)
  }

  const schemaDir = path.dirname(datasource.sourceFilePath)
  const url = getConnectionUrl(datasource)

  // url exists because `ignoreEnvVarErrors: false` would have thrown an error if not
  const canConnect = await canConnectToDatabase(url, schemaDir)

  if (canConnect === true) {
    return true
  } else {
    const { code, message } = canConnect
    throw new Error(`${code}: ${message}`)
  }

}


function getConnectionUrl(datasource: DataSource): string {
  const url = getEffectiveUrl(datasource)
  if (!url.value) {
    if (url.fromEnvVar) {
      throw new Error(`Environment variable '${url.fromEnvVar}' with database connection URL was not found.`)
    } else {
      throw new Error(`Datasource is missing a database connection URL.`)
    }
  }
  return url.value
}

export function getEffectiveUrl(ds: DataSource): EnvValue {
  if (ds.directUrl !== undefined) return ds.directUrl

  return ds.url
}
 */
