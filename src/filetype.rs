pub struct FileType {         
	name: String,
	hl_opts: HighlightingOptions
}
#[derive(Default, Clone)]
pub struct HighlightingOptions {
	numbers: bool,
	strings: bool,
	characters: bool,
	comments: bool,
	multiline_comments: bool,
	primary_keywords: Vec<String>,
	secondary_keywords: Vec<String>,
}

impl HighlightingOptions {
	pub fn numbers(&self) -> bool {
		self.numbers
	}
	
	pub fn strings(&self) -> bool {
		self.strings
	}
	
	pub fn characters(&self) -> bool {
		self.characters
	}
	
	pub fn comments(&self) -> bool {
		self.comments
	}
	
	pub fn multiline_comments(&self) -> bool {
		self.multiline_comments
	}
	
	pub fn primary_keywords(&self) -> &Vec<String> {
		&self.primary_keywords
	}
	
	pub fn secondary_keywords(&self) -> &Vec<String> {
		&self.secondary_keywords
	}
}

impl Default for FileType {
	fn default() -> Self {
		Self { 
			name: String::from("No filetype"),
			hl_opts: HighlightingOptions::default(),
		}
	}
}
impl FileType {
	pub fn name(&self) -> String {
		self.name.clone()
	}
	
	pub fn highlighting_options(&self) -> &HighlightingOptions {
		&self.hl_opts
	}
	
	pub fn from(file_name: &str) -> Self {
		if file_name.ends_with(".mar") {
			return Self {
				name: String::from("Mar"),
				hl_opts: HighlightingOptions { 
					numbers: true,
					strings: true,
					characters: true,
					comments: true,
					multiline_comments: true,
					primary_keywords: vec![
						"func".to_string(),
						"use".to_string(),
						"from".to_string(),
						"as".to_string(),
						"for".to_string(),
						"while".to_string(),
						"class".to_string(),
						"parent".to_string(),
						"if".to_string(),
						"elif".to_string(),
						"else".to_string(),
						"True".to_string(),
						"False".to_string(),
						"None".to_string(),
						"return".to_string(),
						"break".to_string(),
						"continue".to_string(),
						"let".to_string(),
					],
					secondary_keywords: vec![
						"int".to_string(),
						"float".to_string(),
						"string".to_string(),
						"length".to_string(),
					],
					
				},
			};
		}
		Self::default()
	}
}
