use alloc::string::String; // alloc crateのString型をインポート
use alloc::string::ToString;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)] // Debug, Clone, PartialEq, Eqトレイトを自動実装(標準トレイトを自動実装してくれる)
pub struct Url {
    url: String,
    host: String,
    port: String,
    path: String,
    searchpart: String,
}

impl Url{ // Url構造体にメソッドを定義する
    pub fn new(url: String) -> Self{
        // コンストラクタ
        // URLをurl field、それ以外を空文字列で初期化する
        // http://<host>:<port>/<path>?<searchpart>
        Self {
            url,
            host: "".to_string(),
            port: "".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        }
    }

    pub fn parse(&mut self) -> Result<Self, String>{ // Resultは成功した場合Okと結果, 失敗した場合はErrとエラー値を返す
        // URLを各要素に分解する
        // 成功の場合Okとパース結果を持つUrl構造体、不正なURLでパースができない場合はErrとURL文字列を返す 
        if !self.is_http(){
            return Err("Only HTTP scheme is supported.".to_string());
        }

        self.host = self.extract_host();
        self.port = self.extract_port();
        self.path = self.extract_path();
        self.searchpart = self.extract_searchpart();
        Ok(self.clone())
    }

    fn is_http(&self) -> bool {
        if self.url.contains("http://"){
            return true;
        }
        false
    }

    fn extract_host(&self) -> String{
        let url_parts: Vec<&str> = self.url.trim_start_matches("http://") // 前方のhttp://を削除する
        .splitn(2, "/") // :で2つに分割する
        .collect(); // splitnで得られたイテレータを文字列に変換する

        if let Some(index) = url_parts[0].find(":"){ // findは引数文字列が最初に見つかったインデックスを返す. 見つからない場合はNoneのためSome型にしている
            url_parts[0][..index].to_string() // <host>:<port>の<host>部のみ文字列にする
        }else{
            url_parts[0].to_string()
        }
    }

    fn extract_port(&self) -> String{
        let url_parts: Vec<&str> = self.url.trim_start_matches("http://") // 前方のhttp://を削除する
        .splitn(2, "/") // :で2つに分割する
        .collect(); // splitnで得られたイテレータを文字列に変換する

        if let Some(index) = url_parts[0].find(":"){ // findは引数文字列が最初に見つかったインデックスを返す. 見つからない場合はdefault portである80を返す
            url_parts[0][index+1..].to_string() // <host>:<port>の<port>部のみ文字列にする
        }else{
            "80".to_string()
        }
    }
    
    fn extract_path(&self) -> String{
        let url_parts: Vec<&str> = self.url.trim_start_matches("http://") // 前方のhttp://を削除する
        .splitn(2, "/") // :で2つに分割する
        .collect(); // splitnで得られたイテレータを文字列に変換する

        if url_parts.len() < 2{
            return "".to_string();
        }
        let path_and_searchpart: Vec<&str> = url_parts[1]
        .splitn(2, "?")
        .collect();
        path_and_searchpart[0].to_string()
    }
    
    fn extract_searchpart(&self) -> String{
        let url_parts: Vec<&str> = self.url.trim_start_matches("http://") // 前方のhttp://を削除する
        .splitn(2, "/") // :で2つに分割する
        .collect(); // splitnで得られたイテレータを文字列に変換する

        if url_parts.len() < 2{
            return "".to_string();
        }
        let path_and_searchpart: Vec<&str> = url_parts[1]
        .splitn(2, "?")
        .collect();
        if path_and_searchpart.len() < 2{
            "".to_string()
        }else{
            path_and_searchpart[1].to_string()
        }
    }

    pub fn host(&self) -> String{
        self.host.clone()
    }

    pub fn port(&self) -> String{
        self.port.clone()
    }

    pub fn path(&self) -> String{
        self.path.clone()
    }

    pub fn searchpart(&self) -> String{
        self.searchpart.clone()
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_url_host(){
        let url = "http://example.com".to_string();
        let expected = Ok(Url{
            url: url.clone(),
            host:"example.com".to_string(),
            port: "80".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        });
        assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn test_url_host_port(){
        let url = "http://example.com:8888".to_string();
        let expected = Ok(Url{
            url: url.clone(),
            host:"example.com".to_string(),
            port: "8888".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        });
        assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn test_url_host_port_path(){
        let url = "http://example.com:8888/index.html".to_string();
        let expected = Ok(Url{
            url: url.clone(),
            host:"example.com".to_string(),
            port: "8888".to_string(),
            path: "index.html".to_string(),
            searchpart: "".to_string(),
        });
        assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn test_url_host_path(){
        let url = "http://example.com/index.html".to_string();
        let expected = Ok(Url{
            url: url.clone(),
            host:"example.com".to_string(),
            port: "80".to_string(),
            path: "index.html".to_string(),
            searchpart: "".to_string(),
        });
        assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn test_url_host_port_path_searchquery(){
        let url = "http://example.com:8888/index.html?a=123&b=456".to_string();
        let expected = Ok(Url{
            url: url.clone(),
            host:"example.com".to_string(),
            port: "8888".to_string(),
            path: "index.html".to_string(),
            searchpart: "a=123&b=456".to_string(),
        });
        assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn test_no_scheme(){
        let url = "example.com".to_string();
        let expected = Err("Only HTTP scheme is supported.".to_string());
        assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn test_unsupported_scheme(){
        let url = "https://example.com:8888/index.html".to_string();
        let expected = Err("Only HTTP scheme is supported.".to_string());
        assert_eq!(expected, Url::new(url).parse());
    }
}