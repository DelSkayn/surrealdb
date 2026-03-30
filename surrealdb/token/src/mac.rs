#[macro_export]
macro_rules! T {
	(;) => {
		$crate::BaseTokenKind::SemiColon
	};
	(,) => {
		$crate::BaseTokenKind::Comma
	};
	(@) => {
		$crate::BaseTokenKind::At
	};
	(/) => {
		$crate::BaseTokenKind::Slash
	};
	(%) => {
		$crate::BaseTokenKind::Percent
	};
	(|) => {
		$crate::BaseTokenKind::HLine
	};
	(||) => {
		$crate::BaseTokenKind::HLineHLine
	};
	(|>) => {
		$crate::BaseTokenKind::HLineRightShevron
	};

	(&&) => {
		$crate::BaseTokenKind::AndAnd
	};

	(.) => {
		$crate::BaseTokenKind::Dot
	};
	(..) => {
		$crate::BaseTokenKind::DotDot
	};
	(...) => {
		$crate::BaseTokenKind::DotDotDot
	};

	(!) => {
		$crate::BaseTokenKind::Exclaim
	};
	(!=) => {
		$crate::BaseTokenKind::ExclaimEq
	};

	(?) => {
		$crate::BaseTokenKind::Question
	};
	(??) => {
		$crate::BaseTokenKind::QuestionQuestion
	};
	(?=) => {
		$crate::BaseTokenKind::QuestionEqual
	};
	(?:) => {
		$crate::BaseTokenKind::QuestionColon
	};

	(<) => {
		$crate::BaseTokenKind::LeftShevron
	};
	(<~) => {
		$crate::BaseTokenKind::LeftShevronTilde
	};
	(<=) => {
		$crate::BaseTokenKind::LeftShevronEqual
	};
	(<|) => {
		$crate::BaseTokenKind::LeftShevronHLine
	};

	(>) => {
		$crate::BaseTokenKind::RightShevron
	};
	(>=) => {
		$crate::BaseTokenKind::RightShevronEqual
	};

	(-) => {
		$crate::BaseTokenKind::Dash
	};
	(-=) => {
		$crate::BaseTokenKind::DashEqual
	};
	(->) => {
		$crate::BaseTokenKind::DashRightShevron
	};

	(+) => {
		$crate::BaseTokenKind::Plus
	};
	(+=) => {
		$crate::BaseTokenKind::PlusEqual
	};
	(+?=) => {
		$crate::BaseTokenKind::PlusQuestionEqual
	};

	(*) => {
		$crate::BaseTokenKind::Star
	};
	(*=) => {
		$crate::BaseTokenKind::StarEqual
	};
	(**) => {
		$crate::BaseTokenKind::StarStar
	};

	(=) => {
		$crate::BaseTokenKind::Equal
	};
	(==) => {
		$crate::BaseTokenKind::EqualEqual
	};

	(:) => {
		$crate::BaseTokenKind::Colon
	};
	(::) => {
		$crate::BaseTokenKind::ColonColon
	};

	($) => {
		$crate::BaseTokenKind::Dollar
	};

	(ACCESS) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Access)
	};
	(AFTER) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::After)
	};
	(ALGORITHM) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Algorithm)
	};
	(ALL) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::All)
	};
	(ALTER) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Alter)
	};
	(ALWAYS) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Always)
	};
	(ANALYZE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Analyze)
	};
	(ANALYZER) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Analyzer)
	};
	(API) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Api)
	};
	(AS) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::As)
	};
	(ASCENDING) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Ascending)
	};
	(ASCII) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Ascii)
	};
	(ASSERT) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Assert)
	};
	(AT) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::At)
	};
	(AUTHENTICATE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Authenticate)
	};
	(AUTO) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Auto)
	};
	(ASYNC) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Async)
	};
	(BACKEND) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Backend)
	};
	(BATCH) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Batch)
	};
	(BEARER) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Bearer)
	};
	(BEFORE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Before)
	};
	(BEGIN) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Begin)
	};
	(BLANK) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Blank)
	};
	(BM25) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Bm25)
	};
	(BREAK) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Break)
	};
	(BUCKET) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Bucket)
	};
	(BY) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::By)
	};
	(CAMEL) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Camel)
	};
	(CANCEL) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Cancel)
	};
	(CASCADE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Cascade)
	};
	(CHANGEFEED) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::ChangeFeed)
	};
	(CHANGES) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Changes)
	};
	(CAPACITY) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Capacity)
	};
	(CLASS) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Class)
	};
	(CONCURRENTLY) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Concurrently)
	};
	(COMMENT) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Comment)
	};
	(COMMIT) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Commit)
	};
	(COMPLEXITY) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Complexity)
	};
	(COMPACT) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Compact)
	};
	(COMPUTED) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Computed)
	};
	(CONFIG) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Config)
	};
	(CONTENT) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Content)
	};
	(CONTINUE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Continue)
	};
	(COUNT) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Count)
	};
	(CREATE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Create)
	};
	(DATABASE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Database)
	};
	(DEFAULT) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Default)
	};
	(DEFINE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Define)
	};
	(DELETE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Delete)
	};
	(DEPTH) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Depth)
	};
	(DESCENDING) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Descending)
	};
	(DIFF) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Diff)
	};
	(DIMENSION) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Dimension)
	};
	(DISTANCE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Distance)
	};
	(DOC_IDS_CACHE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::DocIdsCache)
	};
	(DOC_IDS_ORDER) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::DocIdsOrder)
	};
	(DOC_LENGTHS_CACHE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::DocLengthsCache)
	};
	(DOC_LENGTHS_ORDER) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::DocLengthsOrder)
	};
	(DROP) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Drop)
	};
	(DUPLICATE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Duplicate)
	};
	(EDGENGRAM) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Edgengram)
	};
	(EFC) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Efc)
	};
	(EVENT) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Event)
	};
	(ELSE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Else)
	};
	(END) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::End)
	};
	(ENFORCED) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Enforced)
	};
	(EXCLUDE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Exclude)
	};
	(EXISTS) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Exists)
	};
	(EXPIRED) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Expired)
	};
	(EXPLAIN) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Explain)
	};
	(EXPUNGE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Expunge)
	};
	(EXTEND_CANDIDATES) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::ExtendCandidates)
	};
	(false) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::False)
	};
	(FETCH) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Fetch)
	};
	(FIELD) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Field)
	};
	(FIELDS) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Fields)
	};
	(FILTERS) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Filters)
	};
	(FLEXIBLE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Flexible)
	};
	(FOR) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::For)
	};
	(FORMAT) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Format)
	};
	(FROM) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::From)
	};
	(FULL) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Full)
	};
	(FULLTEXT) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Fulltext)
	};
	(FUNCTION) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Function)
	};
	(FUNCTIONS) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Functions)
	};
	(GRANT) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Grant)
	};
	(GRAPHQL) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Graphql)
	};
	(GROUP) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Group)
	};
	(HASHED_VECTOR) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::HashedVector)
	};
	(HEADERS) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Headers)
	};
	(HIGHLIGHTS) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Highlights)
	};
	(HNSW) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Hnsw)
	};
	(IGNORE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Ignore)
	};
	(INCLUDE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Include)
	};
	(INCLUSIVE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Inclusive)
	};
	(INDEX) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Index)
	};
	(INFO) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Info)
	};
	(INSERT) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Insert)
	};
	(INTO) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Into)
	};
	(INTROSPECTION) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Introspection)
	};
	(IF) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::If)
	};
	(IS) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Is)
	};
	(ISSUER) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Issuer)
	};
	(JWT) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Jwt)
	};
	(JWKS) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Jwks)
	};
	(KEY) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Key)
	};
	(KEEP_PRUNED_CONNECTIONS) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::KeepPrunedConnections)
	};
	(KILL) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Kill)
	};
	(LET) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Let)
	};
	(LIMIT) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Limit)
	};
	(LIVE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Live)
	};
	(LOWERCASE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Lowercase)
	};
	(LM) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Lm)
	};
	(M) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::M)
	};
	(M0) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::M0)
	};
	(MAPPER) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Mapper)
	};
	(MAXDEPTH) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Maxdepth)
	};
	(MIDDLEWARE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Middleware)
	};
	(ML) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::ML)
	};
	(MERGE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Merge)
	};
	(MODEL) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Model)
	};
	(MODULE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Module)
	};
	(MTREE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::MTree)
	};
	(MTREE_CACHE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::MTreeCache)
	};
	(NAMESPACE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Namespace)
	};
	(NGRAM) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Ngram)
	};
	(NO) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::No)
	};
	(NOINDEX) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::NoIndex)
	};
	(NONE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::None)
	};
	(NULL) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Null)
	};
	(NUMERIC) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Numeric)
	};
	(OMIT) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Omit)
	};
	(ON) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::On)
	};
	(ONLY) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Only)
	};
	(OPTION) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Option)
	};
	(ORDER) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Order)
	};
	(ORIGINAL) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Original)
	};
	(OVERWRITE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Overwrite)
	};
	(PARALLEL) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Parallel)
	};
	(PARAM) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Param)
	};
	(PASSHASH) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Passhash)
	};
	(PASSWORD) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Password)
	};
	(PATCH) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Patch)
	};
	(PATH) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Path)
	};
	(PERMISSIONS) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Permissions)
	};
	(POSTINGS_CACHE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::PostingsCache)
	};
	(POSTINGS_ORDER) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::PostingsOrder)
	};
	(PREPARE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Prepare)
	};
	(PUNCT) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Punct)
	};
	(PURGE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Purge)
	};
	(RANGE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Range)
	};
	(READONLY) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Readonly)
	};
	(REJECT) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Reject)
	};
	(RELATE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Relate)
	};
	(RELATION) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Relation)
	};
	(REBUILD) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Rebuild)
	};
	(REFERENCE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Reference)
	};
	(REFRESH) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Refresh)
	};
	(REMOVE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Remove)
	};
	(REPLACE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Replace)
	};
	(RETRY) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Retry)
	};
	(RETURN) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Return)
	};
	(REVOKE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Revoke)
	};
	(REVOKED) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Revoked)
	};
	(ROLES) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Roles)
	};
	(ROOT) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Root)
	};
	(SCHEMAFULL) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Schemafull)
	};
	(SCHEMALESS) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Schemaless)
	};
	(SCOPE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Scope)
	};
	(SEARCH) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Search)
	};
	(SELECT) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Select)
	};
	(SEQUENCE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Sequence)
	};
	(SESSION) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Session)
	};
	(SET) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Set)
	};
	(SHOW) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Show)
	};
	(SHORTEST) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Shortest)
	};
	(SIGNIN) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Signin)
	};
	(SIGNUP) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Signup)
	};
	(SINCE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Since)
	};
	(SLEEP) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Sleep)
	};
	(SNOWBALL) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Snowball)
	};
	(SPLIT) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Split)
	};
	(START) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Start)
	};
	(STRICT) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Strict)
	};
	(STRUCTURE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Structure)
	};
	(SYSTEM) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::System)
	};
	(TABLE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Table)
	};
	(TABLES) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Tables)
	};
	(TEMPFILES) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::TempFiles)
	};
	(TERMS_CACHE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::TermsCache)
	};
	(TERMS_ORDER) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::TermsOrder)
	};
	(TEXT) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Text)
	};
	(THEN) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Then)
	};
	(THROW) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Throw)
	};
	(TIMEOUT) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Timeout)
	};
	(TO) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::To)
	};
	(TOKENIZERS) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Tokenizers)
	};
	(TOKEN) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Token)
	};
	(TRANSACTION) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Transaction)
	};
	(QUERY_TIMEOUT) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::QueryTimeout)
	};
	(true) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::True)
	};
	(TYPE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Type)
	};
	(UNIQUE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Unique)
	};
	(UNSET) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Unset)
	};
	(UPDATE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Update)
	};
	(UPSERT) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Upsert)
	};
	(UPPERCASE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Uppercase)
	};
	(URL) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Url)
	};
	(USE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Use)
	};
	(USER) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::User)
	};
	(VALUE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Value)
	};
	(VALUES) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Values)
	};
	(VERSION) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Version)
	};
	(VS) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Vs)
	};
	(WHEN) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::When)
	};
	(WHERE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Where)
	};
	(WITH) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::With)
	};
	(ALLINSIDE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::AllInside)
	};
	(ANDKW) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::AndKw)
	};
	(ANYINSIDE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::AnyInside)
	};
	(INSIDE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Inside)
	};
	(INTERSECTS) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Intersects)
	};
	(JSON) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Json)
	};
	(NONEINSIDE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::NoneInside)
	};
	(NOTINSIDE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::NotInside)
	};
	(OR) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::OrKw)
	};
	(OUTSIDE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Outside)
	};
	(NOT) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Not)
	};
	(AND) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::And)
	};
	(COLLATE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Collate)
	};
	(CONTAINSALL) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::ContainsAll)
	};
	(CONTAINSANY) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::ContainsAny)
	};
	(CONTAINSNONE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::ContainsNone)
	};
	(CONTAINSNOT) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::ContainsNot)
	};
	(CONTAINS) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Contains)
	};
	(IN) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::In)
	};
	(OUT) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Out)
	};
	(NORMAL) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Normal)
	};

	// Types
	(ANY) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Any)
	};
	(ARRAY) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Array)
	};
	(GEOMETRY) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Geometry)
	};
	(RECORD) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Record)
	};
	(BOOL) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Bool)
	};
	(BYTES) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Bytes)
	};
	(DATETIME) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Datetime)
	};
	(DECIMAL) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Decimal)
	};
	(DURATION) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Duration)
	};
	(FLOAT) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Float)
	};
	(INT) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Int)
	};
	(NUMBER) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Number)
	};
	(OBJECT) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Object)
	};
	(REGEX) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Regex)
	};
	(STRING) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::String)
	};
	(UUID) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Uuid)
	};
	(ULID) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Ulid)
	};
	(RAND) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Rand)
	};
	(REFERENCES) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::References)
	};
	(FEATURE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Feature)
	};
	(LINE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Line)
	};
	(POINT) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Point)
	};
	(POLYGON) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Polygon)
	};
	(MULTIPOINT) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::MultiPoint)
	};
	(MULTILINE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::MultiLine)
	};
	(MULTIPOLYGON) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::MultiPolygon)
	};
	(COLLECT) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Collect)
	};
	(COLLECTION) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Collection)
	};
	(FILE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::File)
	};

	// Languages
	(ARABIC) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Arabic)
	};
	(DANISH) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Danish)
	};
	(DUTCH) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Dutch)
	};
	(ENGLISH) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::English)
	};
	(FINISH) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Finnish)
	};
	(FRANCH) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::French)
	};
	(GERMAN) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::German)
	};
	(GREEK) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Greek)
	};
	(HUNGRARIAN) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Hungarian)
	};
	(ITALIAN) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Italian)
	};
	(NORWEGIAN) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Norwegian)
	};
	(PORTUGUESE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Portuguese)
	};
	(ROMANIAN) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Romanian)
	};
	(RUSSIAN) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Russian)
	};
	(SPANISH) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Spanish)
	};
	(SWEDISH) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Swedish)
	};
	(TAMIL) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Tamil)
	};
	(TURKISH) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Turkish)
	};

	// Algorithms
	(EDDSA) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::EdDSA)
	};
	(ES256) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Es256)
	};
	(ES384) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Es384)
	};
	(ES512) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Es512)
	};
	(HS256) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Hs256)
	};
	(HS384) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Hs384)
	};
	(HS512) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Hs512)
	};
	(PS256) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Ps256)
	};
	(PS384) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Ps384)
	};
	(PS512) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Ps512)
	};
	(RS256) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Rs256)
	};
	(RS384) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Rs384)
	};
	(RS512) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Rs512)
	};

	// Distance
	(CHEBYSHEV) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Chebyshev)
	};
	(COSINE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Cosine)
	};
	(EUCLIDEAN) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Euclidean)
	};
	(JACCARD) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Jaccard)
	};
	(HAMMING) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Hamming)
	};
	(MANHATTAN) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Manhattan)
	};
	(MINKOWSKI) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Minkowski)
	};
	(PEARSON) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Pearson)
	};

	// VectorTypes
	(F64) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::F64)
	};
	(F32) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::F32)
	};
	(I64) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::I64)
	};
	(I32) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::I32)
	};
	(I16) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::I16)
	};

	// HTTP methods
	(GET) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Get)
	};
	(POST) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Post)
	};
	(PUT) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Put)
	};
	(TRACE) => {
		$crate::BaseTokenKind::Ident($crate::Keyword::Trace)
	};
}
