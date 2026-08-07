

# FreeAgent CLI

Una interfaz de línea de comandos completa para la API de contabilidad de FreeAgent, escrita en Rust. Esta CLI proporciona acceso completo a todos los endpoints de la API de FreeAgent con autenticación OAuth2 integrada y actualización automática de tokens.

## Características

- **Cobertura completa de la API**: Todos los endpoints de la API de FreeAgent, incluyendo facturas, facturas de proveedores, gastos, contactos, proyectos, cuentas bancarias, informes contables, declaraciones de IVA, y más
- **Autenticación OAuth2**: Flujo completo de OAuth2 con PKCE para mayor seguridad
- **Actualización automática de tokens**: Los tokens se actualizan automáticamente cuando expiran
- **Almacenamiento seguro de tokens**: Los tokens se almacenan en el directorio de configuración específico de la plataforma
- **Credenciales integradas**: El ID/secret del cliente OAuth puede integrarse en tiempo de compilación
- **Múltiples formatos de salida**: JSON (predeterminado), tabla o JSON compacto
- **Soporte para entorno de pruebas (sandbox)**: Prueba contra el entorno sandbox de FreeAgent
- **Binario único**: Sin dependencias en tiempo de ejecución, fácil de distribuir

## Instalación

### Instalación rápida (macOS + Linux)

```bash
curl -fsSL https://raw.githubusercontent.com/amogower/freeagent-cli/main/scripts/install.sh | bash
```

Instalar una versión específica:

```bash
curl -fsSL https://raw.githubusercontent.com/amogower/freeagent-cli/main/scripts/install.sh | bash -s -- 0.1.0
```

Usar un fork o un directorio de instalación personalizado:

```bash
FREEAGENT_GITHUB_REPO="yourname/freeagent-cli" INSTALL_DIR="$HOME/.local/bin" \
  curl -fsSL https://raw.githubusercontent.com/amogower/freeagent-cli/main/scripts/install.sh | bash
```

### Homebrew (macOS + Linux)

Este proyecto incluye una plantilla de fórmula de Homebrew en `packaging/homebrew/freeagent.rb`.
Para publicar a través de Homebrew, crea un repositorio tap y copia la fórmula allí después de cada lanzamiento.

### Paquetes para Linux (.deb / .rpm)

Descarga el recurso `.deb` o `.rpm` desde la página de GitHub Releases e instálalo con:

```bash
sudo dpkg -i freeagent_0.1.0_amd64.deb
```

```bash
sudo rpm -i freeagent-0.1.0-1.x86_64.rpm
```

### Instalador para macOS (.pkg)

Descarga el recurso `.pkg` desde la página de GitHub Releases e instálalo con:

```bash
sudo installer -pkg freeagent-0.1.0-aarch64-apple-darwin.pkg -target /
```

### Desde el código fuente

```bash
# Clonar el repositorio
git clone https://github.com/amogower/freeagent-cli
cd freeagent-cli

# Compilar con tus credenciales OAuth
FREEAGENT_CLIENT_ID="your_client_id" \
FREEAGENT_CLIENT_SECRET="your_client_secret" \
cargo build --release

# El binario se encuentra en target/release/freeagent
```

### Instalación con Cargo

```bash
cargo install --git https://github.com/amogower/freeagent-cli
```

### Recursos del lanzamiento (Release Assets)

Cada lanzamiento de GitHub incluye:
- Tarballs por plataforma (`freeagent-<version>-<target>.tar.gz`)
- Instaladores `.pkg` para macOS
- Paquetes `.deb` y `.rpm` para Linux

## Proceso de lanzamiento

Los lanzamientos se automatizan mediante GitHub Actions. Usa el flujo de trabajo `prepare-release` para incrementar la versión, hacer commit y crear la etiqueta, luego la canalización de etiquetas publica los recursos.

1. En GitHub Actions, ejecuta el flujo de trabajo `prepare-release` y elige `major`,
   `minor` o `patch`.
2. El flujo de trabajo ejecuta `scripts/release.js`, el cual actualiza `Cargo.toml` (y
   `Cargo.lock`), hace commit de `chore(release): vX.Y.Z` y push de la etiqueta `vX.Y.Z`.
3. El flujo de trabajo `release` se activa con la etiqueta y carga los recursos en el GitHub
   Release.

Si prefieres etiquetar manualmente, asegúrate de que la etiqueta coincida con la versión en
`Cargo.toml` y haz push de la etiqueta:

```bash
git tag v0.1.0
git push origin v0.1.0
```

El flujo de trabajo compila todos los objetivos, empaqueta los recursos, genera `SHA256SUMS` y produce
`dist/freeagent.rb` para Homebrew (cópialo en tu repositorio tap).


## Documentación

La referencia detallada de comandos se encuentra en `docs/` con un archivo markdown por cada comando de nivel superior.

- Ejecuta `freeagent <command> --help` para un uso rápido
- Consulta `docs/<command>.md` para ver todos los detalles de subcomandos y banderas

## Autocompletado de shell

Genera los autocompletados para tu shell. Elige la ruta que coincida con tu sistema operativo y shell.

### macOS (Intel)

```bash
# Bash
sudo freeagent completions bash > /usr/local/etc/bash_completion.d/freeagent

# Zsh
sudo freeagent completions zsh > /usr/local/share/zsh/site-functions/_freeagent

# Fish
freeagent completions fish > ~/.config/fish/completions/freeagent.fish
```

### macOS (Apple Silicon)

```bash
# Bash
sudo freeagent completions bash > /opt/homebrew/etc/bash_completion.d/freeagent

# Zsh
sudo freeagent completions zsh > /opt/homebrew/share/zsh/site-functions/_freeagent

# Fish
freeagent completions fish > ~/.config/fish/completions/freeagent.fish
```

### Linux

```bash
# Bash (a nivel de sistema)
sudo freeagent completions bash > /etc/bash_completion.d/freeagent

# Zsh (a nivel de sistema)
sudo freeagent completions zsh > /usr/share/zsh/site-functions/_freeagent

# Fish (por usuario)
freeagent completions fish > ~/.config/fish/completions/freeagent.fish
```

### Banderas estructuradas vs --data

La mayoría de las operaciones de escritura aceptan banderas estructuradas con validación. Usa `--data` solo cuando necesites pasar campos que no estén expuestos como banderas.

## Configuración

Esta CLI está configurada para OAuth de fábrica. Si estás compilando desde el código fuente y necesitas proporcionar tus propias credenciales, puedes ofrecerlas en tiempo de compilación o en tiempo de ejecución:

```bash
# Integración en tiempo de compilación (recomendado para distribución)
FREEAGENT_CLIENT_ID="your_id" FREEAGENT_CLIENT_SECRET="your_secret" cargo build --release

# Credenciales en tiempo de ejecución (útil con binarios precompilados)
FREEAGENT_CLIENT_ID="your_id" FREEAGENT_CLIENT_SECRET="your_secret" freeagent login
```

## Uso

### Autenticación

```bash
# Iniciar sesión en FreeAgent (abre el navegador para OAuth)
freeagent login

# Iniciar sesión en el entorno sandbox
freeagent login --sandbox

# Verificar estado de autenticación
freeagent status

# Cerrar sesión
freeagent logout
```

### Empresa

```bash
# Obtener detalles de la empresa
freeagent company get

# Obtener cronograma fiscal
freeagent company tax-timeline
```

### Contactos

```bash
# Listar todos los contactos
freeagent contacts list

# Listar con filtros
freeagent contacts list --view clients --sort name

# Obtener un contacto específico
freeagent contacts get <contact_id>

# Crear un contacto
freeagent contacts create \
  --first-name "John" \
  --last-name "Doe" \
  --email "john@example.com" \
  --organisation-name "Acme Corp"

# Actualizar un contacto
freeagent contacts update <contact_id> --phone-number "+1234567890"

# Eliminar un contacto
freeagent contacts delete <contact_id> --yes
```

### Facturas

```bash
# Listar todas las facturas
freeagent invoices list

# Listar con filtros
freeagent invoices list --view open --from-date 2024-01-01

# Obtener una factura
freeagent invoices get <invoice_id>

# Crear una factura
freeagent invoices create \
  --contact "https://api.freeagent.com/v2/contacts/123" \
  --dated-on "2024-01-15" \
  --payment-terms-in-days 30

# Obtener URL del PDF de la factura
freeagent invoices pdf <invoice_id>

# Enviar factura por correo electrónico
freeagent invoices send-email <invoice_id> --email-to "client@example.com"

# Marcar como enviada
freeagent invoices mark-as-sent <invoice_id>
```

### Facturas de proveedores

```bash
# Listar facturas de proveedores
freeagent bills list --view open

# Crear una factura de proveedor
freeagent bills create \
  --contact "https://api.freeagent.com/v2/contacts/123" \
  --dated-on "2024-01-15" \
  --total-value "500.00"
```

### Gastos

```bash
# Listar gastos
freeagent expenses list --from-date 2024-01-01

# Crear un gasto
freeagent expenses create \
  --user "https://api.freeagent.com/v2/users/1" \
  --category "https://api.freeagent.com/v2/categories/285" \
  --dated-on "2024-01-15" \
  --gross-value "50.00" \
  --description "Office supplies"
```

### Proyectos

```bash
# Listar proyectos
freeagent projects list --view active

# Crear un proyecto
freeagent projects create \
  --contact "https://api.freeagent.com/v2/contacts/123" \
  --name "Website Redesign" \
  --budget "5000" \
  --budget-units hours
```

### Registros de tiempo

```bash
# Listar registros de tiempo
freeagent timeslips list --from-date 2024-01-01

# Crear un registro de tiempo
freeagent timeslips create \
  --user "https://api.freeagent.com/v2/users/1" \
  --project "https://api.freeagent.com/v2/projects/123" \
  --task "https://api.freeagent.com/v2/tasks/456" \
  --dated-on "2024-01-15" \
  --hours "2.5"

# Iniciar temporizador
freeagent timeslips start-timer <timeslip_id>

# Detener temporizador
freeagent timeslips stop-timer <timeslip_id>
```

### Cuentas bancarias y transacciones

```bash
# Listar cuentas bancarias
freeagent bank-accounts list

# Listar transacciones
freeagent bank-transactions list \
  --bank-account "https://api.freeagent.com/v2/bank_accounts/123" \
  --view unexplained

# Crear una transacción
freeagent bank-transactions create \
  --bank-account "https://api.freeagent.com/v2/bank_accounts/123" \
  --dated-on "2024-01-15" \
  --amount "100.00" \
  --description "Client payment"
```

### Informes contables

```bash
# Balance general
freeagent accounting balance-sheet --date 2024-12-31

# Estado de resultados
freeagent accounting profit-and-loss \
  --from-date 2024-01-01 \
  --to-date 2024-12-31

# Balance de comprobación
freeagent accounting trial-balance --date 2024-12-31

# Flujo de caja
freeagent accounting cashflow \
  --from-date 2024-01-01 \
  --to-date 2024-12-31
```

### Declaraciones de IVA

```bash
# Listar declaraciones de IVA
freeagent vat list

# Obtener una declaración de IVA
freeagent vat get <vat_return_id>

# Marcar como presentada
freeagent vat mark-as-filed <vat_return_id>
```

## Formatos de salida

```bash
# JSON (predeterminado)
freeagent contacts list

# Formato tabla
freeagent contacts list --format table

# JSON compacto (línea única)
freeagent contacts list --format compact
```

## Modo Sandbox

Usa la bandera `--sandbox` para probar contra el entorno sandbox de FreeAgent:

```bash
freeagent --sandbox login
freeagent --sandbox invoices list
```

## Grupos de comandos

| Grupo | Descripción |
|-------|-------------|
| `company` | Detalles de la empresa y cronograma fiscal |
| `users` | Gestión de usuarios |
| `contacts` | Gestión de contactos |
| `projects` | Gestión de proyectos |
| `tasks` | Gestión de tareas |
| `invoices` | Gestión de facturas |
| `bills` | Gestión de facturas de proveedores |
| `expenses` | Gestión de gastos |
| `credit-notes` | Gestión de notas de crédito |
| `estimates` | Gestión de presupuestos |
| `recurring-invoices` | Gestión de facturas recurrentes |
| `bank-accounts` | Gestión de cuentas bancarias |
| `bank-transactions` | Gestión de transacciones bancarias |
| `timeslips` | Registro de tiempo |
| `categories` | Listado de categorías |
| `accounting` | Informes contables |
| `vat` | Gestión de declaraciones de IVA |
| `attachments` | Gestión de adjuntos |
| `notes` | Gestión de notas |
| `capital-assets` | Gestión de activos fijos |
| `stock-items` | Gestión de artículos de inventario |

## Manejo de límites de velocidad (Rate Limits)

La CLI maneja automáticamente los límites de velocidad de la API de FreeAgent:
- **Reintento automático**: Las solicitudes que alcanzan los límites de velocidad (estado 429) se reintentan automáticamente
- **Retroceso exponencial**: Utiliza una estrategia de retroceso exponencial con parámetros configurables
- **Soporte para Retry-After**: Respeta el encabezado `Retry-After` de la API
- **Configurable**: Personaliza el comportamiento de reintento mediante variables de entorno

### Configuración de límites de velocidad

Configura el comportamiento de reintento usando variables de entorno:

```bash
# Número máximo de intentos de reintento (predeterminado: 3)
export FREEAGENT_MAX_RETRIES=5

# Duración inicial de retroceso en segundos (predeterminado: 1)
export FREEAGENT_INITIAL_BACKOFF_SECS=2

# Duración máxima de retroceso en segundos (predeterminado: 60)
export FREEAGENT_MAX_BACKOFF_SECS=120

# Usar retroceso exponencial (predeterminado: true)
export FREEAGENT_EXPONENTIAL_BACKOFF=true
```

### Límites de velocidad de la API de FreeAgent

La API de FreeAgent aplica los siguientes límites:
- **120 solicitudes por minuto** por usuario
- **3600 solicitudes por hora** por usuario
- **15 actualizaciones de token por minuto** por usuario

Cuando se excedan estos límites, la CLI esperará automáticamente y reintentará según la estrategia configurada.

## Almacenamiento de tokens

Los tokens se almacenan en el directorio de configuración específico de la plataforma:
- **Linux**: `~/.config/freeagent-cli/tokens.json`
- **macOS**: `~/Library/Application Support/freeagent-cli/tokens.json`
- **Windows**: `C:\Users\<User>\AppData\Roaming\freeagent-cli\tokens.json`

## Seguridad

- OAuth2 con PKCE para mayor seguridad
- El secret del cliente puede integrarse en tiempo de compilación (práctica estándar para aplicaciones nativas)
- Los tokens se almacenan localmente y nunca se transmiten excepto a FreeAgent
- La actualización automática de tokens evita la exposición de credenciales

## Compilación para diferentes plataformas

```bash
# Linux (plataforma actual)
cargo build --release

# Compilación cruzada para macOS (requiere configuración de compilación cruzada)
cargo build --release --target x86_64-apple-darwin

# Compilación cruzada para Windows
cargo build --release --target x86_64-pc-windows-gnu
```

## Licencia

Licencia MIT
